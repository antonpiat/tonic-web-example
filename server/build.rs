fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .build_server(true)
        .compile_protos(&["../proto/grpc-web.proto"], &["../proto"])?;
    Ok(())
}