fn main() {
    let proto_file = "./proto/register.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("Protobuf compilation failed: {}", e));

    println!("cargo:rerun-if-changed={}", proto_file);
}
