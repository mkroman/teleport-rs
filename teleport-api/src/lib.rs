#![allow(clippy::pedantic, clippy::large_enum_variant)]

pub mod teleport {
    tonic::include_proto!("proto");

    pub mod attestation {
        pub mod v1 {
            tonic::include_proto!("teleport.attestation.v1");
        }
    }

    pub mod usageevents {
        pub mod v1 {
            tonic::include_proto!("teleport.usageevents.v1");
        }
    }
}

pub mod types {
    tonic::include_proto!("types");
}

pub mod wrappers {
    tonic::include_proto!("wrappers");
}

pub mod webauthn {
    tonic::include_proto!("webauthn");
}

pub mod events {
    tonic::include_proto!("events");
}

pub use self::teleport::auth_service_client::AuthServiceClient;
