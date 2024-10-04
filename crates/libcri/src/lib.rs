//! # Kubernetes CRI
//!
//! Generate protobuf code from Kubernetes CRI-API proto using `tonic-build`.
//!
//! References:
//! - [kflansburg/k8s-cri](https://github.com/kflansburg/k8s-cri)
//! - [Manual for `tonic-build`](https://github.com/hyperium/tonic/tree/master/tonic-build)
//! - [Kubernetes CRI-API: `v1.proto`](https://github.com/kubernetes/cri-api/blob/c75ef5b/pkg/apis/runtime/v1/api.proto)
//! - [Kubernetes CRI-API: `v1alpha2.proto`](https://github.com/kubernetes/cri-api/blob/c75ef5b/pkg/apis/runtime/v1alpha2/api.proto)

pub mod v1 {
  tonic::include_proto!("runtime.v1");
}

pub mod v1alpha2 {
  tonic::include_proto!("runtime.v1alpha2");
}