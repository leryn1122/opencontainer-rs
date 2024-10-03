//!
//! [Manual for `tonic-build`](https://github.com/hyperium/tonic/tree/master/tonic-build)

fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::configure()
    .build_server(true)
    .build_client(true)
    .build_transport(true)
    .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .compile_protos(&["resources/proto/v1.proto"], &["resources/proto/"])?;

  tonic_build::configure()
    .build_server(true)
    .build_client(true)
    .build_transport(true)
    .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .compile_protos(&["resources/proto/v1alpha2.proto"], &["resources/proto/"])?;

  Ok(())
}
