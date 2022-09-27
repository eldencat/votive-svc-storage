//! build script to generate .rs from .proto

///generates .rs files in src directory
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_config = tonic_build::configure()
        // .type_attribute("UpdateRequest", "#[derive(Copy)]")
        .type_attribute("UpdateResponse", "#[derive(Eq, Copy)]")
        .type_attribute("GetResponse", "#[derive(Eq, Copy)]")
        .type_attribute("GetRequest", "#[derive(Eq, Copy)]")
        .type_attribute("Signature", "#[derive(Eq, Copy)]")
        .type_attribute("UpdateRequest", "#[derive(Eq)]");
    let client_config = server_config.clone();

    server_config
        .build_client(false)
        .out_dir("src/")
        .compile(&["../proto/grpc.proto"], &["../proto"])?;

    client_config
        .build_server(false)
        .out_dir("../client/src")
        .compile(&["../proto/grpc.proto"], &["../proto"])?;

    Ok(())
}
