fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut config = prost_build::Config::new();
  config.btree_map(&["."]);

  tonic_build::configure()
    .build_server(true)
    .build_client(true)
    .build_transport(true)
    // .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .compile_protos(&["resources/proto/v1.proto"], &["resources/proto/"])?;

  tonic_build::configure()
    .build_server(true)
    .build_client(true)
    .build_transport(true)
    // .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .compile_protos(&["resources/proto/v1alpha2.proto"], &["resources/proto/"])?;

  Ok(())
}
