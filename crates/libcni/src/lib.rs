//! # CNI (Container Networking Interface)
//!
//! References:
//! - [CNI Spec](https://github.com/containernetworking/cni/blob/main/SPEC.md)

use crate::api::CniPlugin;
use crate::schema::error::CniResult;

pub mod api;
pub mod schema;

pub struct VersionInfo;

pub fn plugin_main_entrypoint(
  plugin: impl CniPlugin,
  compatibility: &[VersionInfo],
  build_version: VersionInfo,
) -> CniResult<()> {
  Ok(())
}
