use std::fs;
use std::io::BufReader;
use std::path::Path;

use hyper::{client::HttpConnector, Uri};
use miette::{IntoDiagnostic, WrapErr};
use rustls_pemfile::Item;
use serde::Deserialize;
use tokio_rustls::rustls::{Certificate, ClientConfig, PrivateKey, RootCertStore};
use tracing::{debug, info, trace};

use teleport_api::teleport::PingRequest;
use teleport_api::AuthServiceClient;

mod client;
mod webclient;

const CURRENT_PROFILE_FILENAME: &str = "current-profile";
const PROFILE_DIR: &str = ".tsh";

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
struct Profile {
    pub web_proxy_addr: Option<String>,

    pub ssh_proxy_addr: Option<String>,

    pub kube_proxy_addr: Option<String>,

    pub postgres_proxy_addr: Option<String>,

    pub mysql_proxy_addr: Option<String>,

    pub mongo_proxy_addr: Option<String>,
    /// The username.
    pub user: Option<String>,
    /// The cluster name.
    pub cluster: Option<String>,

    #[serde(rename = "forward_ports")]
    pub forwarded_ports: Option<Vec<String>>,

    #[serde(rename = "dynamic_forward_ports")]
    pub dynamic_forwarded_ports: Option<Vec<String>>,

    pub dir: String,

    pub tls_routing_enabled: Option<bool>,

    pub auth_connector: Option<String>,

    pub load_all_cas: Option<bool>,

    pub mfa_mode: Option<String>,
}

/// Attempts to read the name of the active `tsh` profile, which is stored in
/// `~/.tsh/current-profile`.
///
/// Returns `None` if the file does not exist.
fn current_profile_name(base_dir: &Path) -> Option<String> {
    let path = base_dir.join(CURRENT_PROFILE_FILENAME);
    let profile_name = fs::read_to_string(path).ok()?;

    Some(profile_name.trim().to_string())
}

fn load_profile(base_dir: &Path, profile_name: &str) -> miette::Result<Profile> {
    let profile_path = base_dir.join(format!("{}.yaml", profile_name));

    trace!(?profile_name, ?profile_path, "opening profile");

    let file = fs::File::open(&profile_path)
        .into_diagnostic()
        .with_context(|| format!("Opening profile {:?}", profile_path))?;

    trace!(?profile_name, ?profile_path, "parsing profile");

    let profile = serde_yaml::from_reader(&file)
        .into_diagnostic()
        .context("Could not deserialize profile data")?;

    Ok(profile)
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let tsh_dir = dirs::home_dir()
        .expect("unable to get HOME directory")
        .join(PROFILE_DIR);

    let profile_name =
        current_profile_name(&tsh_dir).expect("a preinitialized tsh profile is required");
    let profile = load_profile(&tsh_dir, profile_name.as_str())?;

    debug!(?profile, "loaded profile");

    let host = profile.web_proxy_addr.unwrap();
    let addr = format!("https://{}", host);

    let user = profile.user.unwrap();
    let cluster = profile.cluster.unwrap();
    let cluster_keys_dir = tsh_dir.join("keys").join(&profile_name);

    let ca_path = cluster_keys_dir
        .join("cas")
        .join(format!("{}.pem", &cluster));
    let cert_path = cluster_keys_dir.join(format!("{}-x509.pem", user));
    let key_path = cluster_keys_dir.join(user);

    info!(?ca_path, ?cert_path, ?key_path, "Loading TLS identity");
    let mut ca_file = BufReader::new(std::fs::File::open(ca_path).into_diagnostic()?);
    let mut cert_file = BufReader::new(std::fs::File::open(cert_path).into_diagnostic()?);
    let mut key_file = BufReader::new(std::fs::File::open(key_path).into_diagnostic()?);

    let ca_cert_der = if let Item::X509Certificate(data) = rustls_pemfile::read_one(&mut ca_file)
        .into_diagnostic()?
        .ok_or_else(|| miette::miette!("Invalid X509 certificate"))?
    {
        data
    } else {
        unimplemented!()
    };

    let cert_der = if let Item::X509Certificate(data) = rustls_pemfile::read_one(&mut cert_file)
        .into_diagnostic()?
        .ok_or_else(|| miette::miette!("Invalid x509 certificate"))?
    {
        data
    } else {
        unimplemented!()
    };
    let key_der = if let Item::RSAKey(data) = rustls_pemfile::read_one(&mut key_file)
        .into_diagnostic()?
        .ok_or_else(|| miette::miette!("Invalid PEM RSA key"))?
    {
        data
    } else {
        unimplemented!()
    };

    let key = PrivateKey(key_der);
    let cert = Certificate(cert_der);
    let ca_cert = Certificate(ca_cert_der);
    let mut root_certs = RootCertStore::empty();

    root_certs.add(&ca_cert).into_diagnostic()?;

    for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs") {
        root_certs.add(&Certificate(cert.0)).unwrap();
    }

    let mut tls = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_certs)
        .with_single_cert(vec![cert], key)
        .into_diagnostic()?;

    tls.alpn_protocols = vec![b"teleport-proxy-ssh".to_vec()];

    let mut http = HttpConnector::new();
    http.enforce_http(false);

    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(tls)
        .https_or_http()
        .enable_http2()
        .build();

    let uri = Uri::from_maybe_shared(addr).into_diagnostic()?;
    let client = hyper::Client::builder().build(connector);

    let mut auth_service = AuthServiceClient::with_origin(client, uri);

    info!(?auth_service);
    let res = auth_service
        .ping(PingRequest::default())
        .await
        .into_diagnostic()?;
    info!(?res);

    Ok(())
}
