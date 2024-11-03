use libcsi::v1::plugin_capability;
use libcsi::v1::GetPluginCapabilitiesRequest;
use libcsi::v1::GetPluginCapabilitiesResponse;
use libcsi::v1::GetPluginInfoRequest;
use libcsi::v1::GetPluginInfoResponse;
use libcsi::v1::PluginCapability;
use libcsi::v1::ProbeRequest;
use libcsi::v1::ProbeResponse;
use tonic::Request;
use tonic::Response;
use tonic::Status;

pub struct IdentityServerImpl {
  name: String,
}

impl IdentityServerImpl {
  pub fn new(name: impl Into<String>) -> Self {
    Self { name: name.into() }
  }
}

#[async_trait::async_trait]
impl libcsi::v1::identity_server::Identity for IdentityServerImpl {
  async fn get_plugin_info(
    &self,
    _: Request<GetPluginInfoRequest>,
  ) -> Result<Response<GetPluginInfoResponse>, Status> {
    Ok(Response::new(GetPluginInfoResponse {
      name: self.name.clone(),
      vendor_version: env!("CARGO_PKG_VERSION").into(),
      ..Default::default()
    }))
  }

  async fn get_plugin_capabilities(
    &self,
    _: Request<GetPluginCapabilitiesRequest>,
  ) -> Result<Response<GetPluginCapabilitiesResponse>, Status> {
    Ok(Response::new(GetPluginCapabilitiesResponse {
      capabilities: vec![PluginCapability {
        r#type: Some(plugin_capability::Type::Service(
          plugin_capability::Service {
            r#type: plugin_capability::service::Type::ControllerService.into(),
          },
        )),
      }],
    }))
  }

  async fn probe(&self, _: Request<ProbeRequest>) -> Result<Response<ProbeResponse>, Status> {
    Ok(Response::new(ProbeResponse { ready: Some(true) }))
  }
}
