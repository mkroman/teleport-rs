fn main() {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .build_transport(true)
        .compile(
            &[
                "proto/teleport/legacy/client/proto/authservice.proto",
                "proto/teleport/legacy/client/proto/joinservice.proto",
                "proto/teleport/legacy/client/proto/proxyservice.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
