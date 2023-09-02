use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "proto/mandos_auth.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(true)
        .build_client(false)
        .file_descriptor_set_path(out_dir.join("mandos_auth_descriptor.bin"))
        .out_dir("./src")
        .compile(&[proto_file], &["proto"])?;

    Ok(())
}
