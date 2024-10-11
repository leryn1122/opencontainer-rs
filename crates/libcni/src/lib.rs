//! # CNI (Container Networking Interface)
//!
//! References:
//! - [CNI Spec](https://github.com/containernetworking/cni/blob/main/SPEC.md)

use crate::api::CniExecution;
use crate::api::CniPlugin;
use crate::schema::error::CniResult;
use crate::schema::NetNS;
use crate::version::check_version;
use crate::version::PluginInfo;

pub mod api;
pub mod plugin;
pub mod schema;
pub mod version;

/// ```rust
/// use libcni::plugin_main_entrypoint;
/// use libcni::schema::error::CniResult;
/// use libcni::version;
///
/// fn main() {
///   plugin_main_entrypoint(plugin, version::All, semver::Version::new(1, 0, 0))
/// }
/// ```
pub fn plugin_main_entrypoint(
  plugin: impl CniPlugin,
  plugin_info: impl PluginInfo,
  build_version: semver::Version,
) {
  plugin_main_entrypoint_with_error(plugin, plugin_info, build_version).map_err(|e| {});
}

fn plugin_main_entrypoint_with_error(
  plugin: impl CniPlugin,
  plugin_info: impl PluginInfo,
  build_version: semver::Version,
) -> CniResult<()> {
  let execution = CniExecution::load_args_from_env()?;

  match execution {
    CniExecution::Add(args) => {
      check_version(&build_version, plugin_info.supported_versions())?;
      plugin.add(args)
      // check_netns(netns)
    }
    CniExecution::Del(args) => {
      check_version(&build_version, plugin_info.supported_versions())?;
      plugin.del(args)
      // check_netns(netns)?;
    }
    CniExecution::Check(args) => plugin.check(args),
    CniExecution::Status => plugin.status(),
    CniExecution::GC(args) => plugin.gc(args),
    CniExecution::Version => plugin.version(),
  }
}

// Plugin's netns and CNI netns should not be the same.
fn check_netns(netns: NetNS) -> CniResult<()> {
  todo!()
}
