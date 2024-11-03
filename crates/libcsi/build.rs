fn main() -> Result<(), Box<dyn std::error::Error>> {
  // println!("cargo:rerun-if-changed=resources/proto/csi.proto");

  let mut config = prost_build::Config::new();
  config.btree_map(&["."]);

  tonic_build::configure()
    .build_server(true)
    .build_client(true)
    .build_transport(true)
    // .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .compile_protos_with_config(config, &["resources/proto/csi.proto"], &["resources/proto/"])?;

  Ok(())
}
