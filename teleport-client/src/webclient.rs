use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SecondFactor {
    Off,
    Otp,
    U2f,
    WebAuthn,
    On,
    Optional,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrivateKeyPolicy {
    None,
    HardwareKey,
    HardwareKeyTouch,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalSettings {
    pub name: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct WebAuthn {
    pub rp_id: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct U2fSettings {
    pub app_id: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct OidcSettings {
    pub name: String,
    pub display: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct SamlSettings {
    pub name: String,
    pub display: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct GithubSettings {
    pub name: String,
    pub display: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct AuthenticationSettings {
    pub r#type: String,
    pub second_factor: Option<SecondFactor>,
    pub preferred_local_mfa: Option<SecondFactor>,
    pub allow_passwordless: Option<bool>,
    pub local: Option<LocalSettings>,
    pub webauthn: Option<WebAuthn>,
    pub u2f: Option<U2fSettings>,
    pub oidc: Option<OidcSettings>,
    pub saml: Option<SamlSettings>,
    pub github: Option<GithubSettings>,
    pub private_key_policy: PrivateKeyPolicy,
    pub has_motd: bool,
    pub load_all_cas: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct SshProxySettings {
    pub listen_addr: Option<String>,

    pub tunnel_listen_addr: Option<String>,

    pub web_listen_addr: Option<String>,

    pub public_addr: Option<String>,

    pub ssh_public_addr: Option<String>,

    pub ssh_tunnel_public_addr: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct DbProxySettings {
    pub postgres_listen_addr: Option<String>,

    pub postgres_public_addr: Option<String>,

    pub mysql_listen_addr: Option<String>,

    pub mysql_public_addr: Option<String>,

    pub mongo_listen_addr: Option<String>,

    pub mongo_public_addr: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct KubeProxySettings {
    pub enabled: Option<bool>,

    pub public_addr: Option<String>,

    pub listen_addr: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxySettings {
    pub ssh: SshProxySettings,
    pub kube: KubeProxySettings,
    pub db: DbProxySettings,
    pub tls_routing_enabled: bool,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct PingResponse {
    pub auth: AuthenticationSettings,
    pub proxy: ProxySettings,
    pub server_version: String,
    pub min_client_version: String,
    pub cluster_name: String,
    pub license_warnings: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_deserialize_enum() {
        let json = r#"
            {
              "auth": {
                "type": "local",
                "second_factor": "otp",
                "preferred_local_mfa": "otp",
                "local": {
                  "name": ""
                },
                "private_key_policy": "none",
                "has_motd": false
              },
              "proxy": {
                "kube": {},
                "ssh": {
                  "listen_addr": "[::]:3023",
                  "tunnel_listen_addr": "0.0.0.0:3024",
                  "web_listen_addr": "0.0.0.0:3080",
                  "public_addr": "cluster-1:3080"
                },
                "db": {},
                "tls_routing_enabled": false
              },
              "server_version": "11.3.1",
              "min_client_version": "10.0.0",
              "cluster_name": "cluster-1"
            }
        "#;

        let deserialized: PingResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.auth.private_key_policy, PrivateKeyPolicy::None);
        assert_eq!(deserialized.auth.second_factor, Some(SecondFactor::Otp));
    }
}
