//! build script to generate .rs from .proto

///generates .rs files in src directory
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the Client
    tonic_build::configure()
        .build_server(false)
        .out_dir("../client/src/")
        .compile(&["../proto/grpc.proto"], &["../proto"])?;

    // Build the Server
    tonic_build::configure()
        .build_client(false)
        .out_dir("src/")
        .compile(&["../proto/grpc.proto"], &["../proto"])?;

    Ok(())
}
