use libcsi::v1::node_service_capability;
use libcsi::v1::NodeExpandVolumeRequest;
use libcsi::v1::NodeExpandVolumeResponse;
use libcsi::v1::NodeGetCapabilitiesRequest;
use libcsi::v1::NodeGetCapabilitiesResponse;
use libcsi::v1::NodeGetInfoRequest;
use libcsi::v1::NodeGetInfoResponse;
use libcsi::v1::NodeGetVolumeStatsRequest;
use libcsi::v1::NodeGetVolumeStatsResponse;
use libcsi::v1::NodePublishVolumeRequest;
use libcsi::v1::NodePublishVolumeResponse;
use libcsi::v1::NodeServiceCapability;
use libcsi::v1::NodeStageVolumeRequest;
use libcsi::v1::NodeStageVolumeResponse;
use libcsi::v1::NodeUnpublishVolumeRequest;
use libcsi::v1::NodeUnpublishVolumeResponse;
use libcsi::v1::NodeUnstageVolumeRequest;
use libcsi::v1::NodeUnstageVolumeResponse;
use tonic::Request;
use tonic::Response;
use tonic::Status;

pub struct NodeServerImpl {
  node_id: String,
}

impl NodeServerImpl {
  pub fn new(node_id: String) -> Self {
    Self { node_id }
  }

  fn get_node_service_capabilities(&self) -> Vec<node_service_capability::rpc::Type> {
    use node_service_capability::rpc::Type;
    vec![
      Type::GetVolumeStats,
      Type::VolumeCondition,
      Type::SingleNodeMultiWriter,
    ]
  }

  fn validate_node_service_capabilities(
    &self,
    capability: node_service_capability::rpc::Type,
  ) -> Result<(), Status> {
    use node_service_capability::rpc::Type;
    if Type::Unknown == capability {
      Err(Status::invalid_argument("Unknown Capability"))?;
    }
    if self.get_node_service_capabilities().contains(&capability) {
      Ok(())
    } else {
      Err(Status::invalid_argument("Unknown Capability"))
    }
  }
}

#[async_trait::async_trait]
impl libcsi::v1::node_server::Node for NodeServerImpl {
  async fn node_stage_volume(
    &self,
    request: Request<NodeStageVolumeRequest>,
  ) -> Result<Response<NodeStageVolumeResponse>, Status> {
    todo!()
  }

  async fn node_unstage_volume(
    &self,
    request: Request<NodeUnstageVolumeRequest>,
  ) -> Result<Response<NodeUnstageVolumeResponse>, Status> {
    todo!()
  }

  async fn node_publish_volume(
    &self,
    request: Request<NodePublishVolumeRequest>,
  ) -> Result<Response<NodePublishVolumeResponse>, Status> {
    todo!()
  }

  async fn node_unpublish_volume(
    &self,
    request: Request<NodeUnpublishVolumeRequest>,
  ) -> Result<Response<NodeUnpublishVolumeResponse>, Status> {
    todo!()
  }

  async fn node_get_volume_stats(
    &self,
    request: Request<NodeGetVolumeStatsRequest>,
  ) -> Result<Response<NodeGetVolumeStatsResponse>, Status> {
    todo!()
  }

  async fn node_expand_volume(
    &self,
    _: Request<NodeExpandVolumeRequest>,
  ) -> Result<Response<NodeExpandVolumeResponse>, Status> {
    Err(Status::unimplemented("Not yet implemented"))
  }

  async fn node_get_capabilities(
    &self,
    _: Request<NodeGetCapabilitiesRequest>,
  ) -> Result<Response<NodeGetCapabilitiesResponse>, Status> {
    Ok(Response::new(NodeGetCapabilitiesResponse {
      capabilities: self
        .get_node_service_capabilities()
        .iter()
        .map(|c| NodeServiceCapability {
          r#type: Some(node_service_capability::Type::Rpc(
            node_service_capability::Rpc {
              r#type: node_service_capability::rpc::Type::StageUnstageVolume.into(),
            },
          )),
        })
        .collect(),
    }))
  }

  async fn node_get_info(
    &self,
    _: Request<NodeGetInfoRequest>,
  ) -> Result<Response<NodeGetInfoResponse>, Status> {
    Ok(Response::new(NodeGetInfoResponse {
      node_id: self.node_id.clone(),
      ..Default::default()
    }))
  }
}
