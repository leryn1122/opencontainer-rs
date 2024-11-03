use libcsi::v1::controller_service_capability;
use libcsi::v1::validate_volume_capabilities_response;
use libcsi::v1::ControllerExpandVolumeRequest;
use libcsi::v1::ControllerExpandVolumeResponse;
use libcsi::v1::ControllerGetCapabilitiesRequest;
use libcsi::v1::ControllerGetCapabilitiesResponse;
use libcsi::v1::ControllerGetVolumeRequest;
use libcsi::v1::ControllerGetVolumeResponse;
use libcsi::v1::ControllerModifyVolumeRequest;
use libcsi::v1::ControllerModifyVolumeResponse;
use libcsi::v1::ControllerPublishVolumeRequest;
use libcsi::v1::ControllerPublishVolumeResponse;
use libcsi::v1::ControllerServiceCapability;
use libcsi::v1::ControllerUnpublishVolumeRequest;
use libcsi::v1::ControllerUnpublishVolumeResponse;
use libcsi::v1::CreateSnapshotRequest;
use libcsi::v1::CreateSnapshotResponse;
use libcsi::v1::CreateVolumeRequest;
use libcsi::v1::CreateVolumeResponse;
use libcsi::v1::DeleteSnapshotRequest;
use libcsi::v1::DeleteSnapshotResponse;
use libcsi::v1::DeleteVolumeRequest;
use libcsi::v1::DeleteVolumeResponse;
use libcsi::v1::GetCapacityRequest;
use libcsi::v1::GetCapacityResponse;
use libcsi::v1::ListSnapshotsRequest;
use libcsi::v1::ListSnapshotsResponse;
use libcsi::v1::ListVolumesRequest;
use libcsi::v1::ListVolumesResponse;
use libcsi::v1::ValidateVolumeCapabilitiesRequest;
use libcsi::v1::ValidateVolumeCapabilitiesResponse;
use libcsi::v1::Volume;
use tonic::Request;
use tonic::Response;
use tonic::Status;

pub struct ControllerServerImpl {
  name: String,
}

impl ControllerServerImpl {
  pub fn new(name: impl Into<String>) -> Self {
    Self { name: name.into() }
  }

  fn get_controller_service_capabilities(&self) -> Vec<controller_service_capability::rpc::Type> {
    use controller_service_capability::rpc::Type;
    vec![
      Type::CreateDeleteVolume,
      Type::GetCapacity,
      Type::PublishUnpublishVolume,
      Type::PublishReadonly,
      Type::ListVolumes,
      Type::SingleNodeMultiWriter,
    ]
  }

  fn validate_controller_service_request_capability(
    &self,
    capability: controller_service_capability::rpc::Type,
  ) -> Result<(), Status> {
    use controller_service_capability::rpc::Type;
    if Type::Unknown == capability {
      Err(Status::invalid_argument("unknown capability"))?;
    }
    if self
      .get_controller_service_capabilities()
      .contains(&capability)
    {
      Ok(())
    } else {
      Err(Status::invalid_argument("unknown capability"))
    }
  }

  fn validate_create_volume_request(&self, request: &CreateVolumeRequest) -> Result<(), Status> {
    Ok(())
  }
}

#[async_trait::async_trait]
impl libcsi::v1::controller_server::Controller for ControllerServerImpl {
  async fn create_volume(
    &self,
    request: Request<CreateVolumeRequest>,
  ) -> Result<Response<CreateVolumeResponse>, Status> {
    self
      .validate_controller_service_request_capability(
        controller_service_capability::rpc::Type::CreateDeleteVolume,
      )
      .map_err(|_| Status::invalid_argument("invalid parameters"))?;

    let request: CreateVolumeRequest = request.into_inner();

    let volume = Volume {
      volume_id: request.name,
      volume_context: Default::default(),
      capacity_bytes: 0,
      ..Default::default()
    };

    Ok(Response::new(CreateVolumeResponse {
      volume: Some(volume),
    }))
  }

  async fn delete_volume(
    &self,
    request: Request<DeleteVolumeRequest>,
  ) -> Result<Response<DeleteVolumeResponse>, Status> {
    let request = request.into_inner();
    let volume_id = request.volume_id;

    Ok(Response::new(DeleteVolumeResponse {}))
  }

  async fn controller_publish_volume(
    &self,
    _: Request<ControllerPublishVolumeRequest>,
  ) -> Result<Response<ControllerPublishVolumeResponse>, Status> {
    Err(Status::unimplemented("Not yet implemented"))
  }

  async fn controller_unpublish_volume(
    &self,
    _: Request<ControllerUnpublishVolumeRequest>,
  ) -> Result<Response<ControllerUnpublishVolumeResponse>, Status> {
    Err(Status::unimplemented("Not yet implemented"))
  }

  async fn validate_volume_capabilities(
    &self,
    request: Request<ValidateVolumeCapabilitiesRequest>,
  ) -> Result<Response<ValidateVolumeCapabilitiesResponse>, Status> {
    let request = request.into_inner();
    if request.volume_id.len() == 0 {
      Err(Status::invalid_argument("volume name missing in request"))?;
    }
    if request.volume_capabilities.is_empty() {
      Err(Status::invalid_argument(
        "volume capabilities cannot be empty",
      ))?;
    }
    Ok(Response::new(ValidateVolumeCapabilitiesResponse {
      confirmed: Some(validate_volume_capabilities_response::Confirmed {
        volume_capabilities: request.volume_capabilities.clone(),
        ..Default::default()
      }),
      ..Default::default()
    }))
  }

  async fn list_volumes(
    &self,
    request: Request<ListVolumesRequest>,
  ) -> Result<Response<ListVolumesResponse>, Status> {
    let request = request.into_inner();

    let starting_token = request.starting_token;
    let start = starting_token
      .parse::<usize>()
      .map_err(|e| Status::aborted(format!("starting_token parse: {}", e)))?;

    Ok(Response::new(ListVolumesResponse {
      entries:    vec![],
      next_token: starting_token,
    }))
  }

  async fn get_capacity(
    &self,
    _: Request<GetCapacityRequest>,
  ) -> Result<Response<GetCapacityResponse>, Status> {
    Ok(Response::new(GetCapacityResponse {
      available_capacity: 0,
      ..Default::default()
    }))
  }

  async fn controller_get_capabilities(
    &self,
    _: Request<ControllerGetCapabilitiesRequest>,
  ) -> Result<Response<ControllerGetCapabilitiesResponse>, Status> {
    Ok(Response::new(ControllerGetCapabilitiesResponse {
      capabilities: self
        .get_controller_service_capabilities()
        .iter()
        .map(|c| ControllerServiceCapability {
          r#type: Some(controller_service_capability::Type::Rpc(
            controller_service_capability::Rpc {
              r#type: (*c).into(),
            },
          )),
        })
        .collect(),
    }))
  }

  async fn create_snapshot(
    &self,
    _: Request<CreateSnapshotRequest>,
  ) -> Result<Response<CreateSnapshotResponse>, Status> {
    Err(Status::unimplemented("Not yet implemented"))
  }

  async fn delete_snapshot(
    &self,
    _: Request<DeleteSnapshotRequest>,
  ) -> Result<Response<DeleteSnapshotResponse>, Status> {
    Err(Status::unimplemented("Not yet implemented"))
  }

  async fn list_snapshots(
    &self,
    _: Request<ListSnapshotsRequest>,
  ) -> Result<Response<ListSnapshotsResponse>, Status> {
    Err(Status::unimplemented("Not yet implemented"))
  }

  async fn controller_expand_volume(
    &self,
    _: Request<ControllerExpandVolumeRequest>,
  ) -> Result<Response<ControllerExpandVolumeResponse>, Status> {
    Err(Status::unimplemented("Not yet implemented"))
  }

  async fn controller_get_volume(
    &self,
    _: Request<ControllerGetVolumeRequest>,
  ) -> Result<Response<ControllerGetVolumeResponse>, Status> {
    if let Err(e) = self.validate_controller_service_request_capability(
      controller_service_capability::rpc::Type::GetVolume,
    ) {
      Err(Status::aborted(e.to_string()))?;
    }
    Err(Status::unimplemented("Not yet implemented"))
  }

  async fn controller_modify_volume(
    &self,
    _: Request<ControllerModifyVolumeRequest>,
  ) -> Result<Response<ControllerModifyVolumeResponse>, Status> {
    Err(Status::unimplemented("Not yet implemented"))
  }
}
