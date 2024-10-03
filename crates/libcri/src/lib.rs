//! # Kubernetes CRI
//! Reference:
//! - [v1.proto](https://github.com/kubernetes/cri-api/blob/c75ef5b/pkg/apis/runtime/v1/api.proto)
//! - [v1alpha2.proto](https://github.com/kubernetes/cri-api/blob/c75ef5b/pkg/apis/runtime/v1alpha2/api.proto)

pub mod v1 {
  tonic::include_proto!("runtime.v1");
}

pub mod v1alpha2 {
  tonic::include_proto!("runtime.v1alpha2");
}