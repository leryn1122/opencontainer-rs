use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use support::semver::deserialize_version;
use support::semver::deserialize_versions;
use support::semver::serialize_version;
use support::semver::serialize_versions;

/// [Configuration format](https://github.com/containernetworking/cni/blob/main/SPEC.md#container-network-interface-cni-specification)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CniNetworkConfig {
  /// Semantic Version 2.0 of CNI specification to which this configuration list and all the
  /// individual configurations conform.
  #[serde(
    deserialize_with = "deserialize_version",
    serialize_with = "serialize_version"
  )]
  pub cni_version: semver::Version,
  #[serde(rename = "type")]
  #[serde(
    deserialize_with = "deserialize_versions",
    serialize_with = "serialize_versions"
  )]
  /// List of all CNI versions which this configuration supports.
  pub cni_versions: Vec<semver::Version>,
  /// Network name. This should be unique across all network configurations on a host
  /// (or other administrative domain).
  pub name: String,
  /// Either true or false.
  pub disable_check: bool,
  /// Either true or false.
  #[serde(rename = "disableGC")]
  pub disable_gc: bool,
  /// Either true or false.
  pub load_only_inlined_plugins: bool,
  /// A list of CNI plugins and their configuration, which is a list of plugin configuration
  /// objects.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub plugins: Option<Vec<PluginConfig>>,
  /*
  Plugin configuration objects:
  */

  /*
  Required keys:
   */
  /// Matches the name of the CNI plugin binary on disk.
  #[serde(rename = "type")]
  pub type_:          String,
  /*
  Optional keys, used by the protocol:
   */
  ///
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub capabilities:   Option<HashMap<String, bool>>,
  /*
  Reserved keys, used by the protocol:
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub runtime_config: Option<RuntimeConfig>,
  ///
  pub args:           Vec<String>,
  /// Any keys starting with `cni.dev/`
  pub attributes:     Option<HashMap<String, String>>,
  /*
  Optional keys, well-known:
  These keys are not used by the protocol, but have a standard meaning to plugins.
  */
  /// If supported by the plugin, sets up an IP masquerade on the host for this network.
  #[serde(default)]
  pub ip_masq:        bool,
  /// Dictionary with IPAM (IP Address Management) specific values
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ipam:           Option<IPAMConfig>,
  /// Dictionary with DNS specific values
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dns:            Option<DNSConfig>,
  /*
  Other keys
   */

  /// Extra values...
  #[serde(flatten)]
  pub extra_values: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeConfig {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginConfig {
  /// Matches the name of the CNI plugin binary on disk.
  #[serde(rename = "type")]
  pub type_:          String,
  ///
  #[serde(skip_serializing_if = "Option::is_none")]
  pub capabilities:   Option<HashMap<String, bool>>,
  ///
  #[serde(skip_serializing_if = "Option::is_none")]
  pub runtime_config: Option<RuntimeConfig>,
  ///
  pub args:           Vec<String>,
  /*
  Other keys
   */

  /// Extra values...
  #[serde(flatten)]
  pub extra_values: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IPAMConfig {
  #[serde(rename = "type")]
  pub type_: String,
}

/// # DNS configuration
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DNSConfig {
  /// list of a priority-ordered list of DNS nameservers that this network is aware of. Each entry
  /// in the list is a string containing either an IPv4 or an IPv6 address.
  pub nameservers: Vec<String>,
  /// the local domain used for short hostname lookups.
  pub domain:      String,
  /// list of priority ordered search domains for short hostname lookups. Will be preferred over
  /// `domain` by most resolvers.
  pub search:      Vec<String>,
  /// list of options that can be passed to the resolver.
  pub options:     Vec<String>,
}

pub struct NetworkConfigList {}
