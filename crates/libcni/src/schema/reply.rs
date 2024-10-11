use std::net::IpAddr;
use std::path::PathBuf;

use ipnetwork::IpNetwork;
use semver::Version;
use serde::Deserialize;
use serde::Serialize;
use support::semver::deserialize_version;
use support::semver::deserialize_versions;
use support::semver::serialize_version;
use support::semver::serialize_versions;

use crate::schema::config::DNSConfig;
use crate::schema::error::CniResult;

pub trait ReplyPayload<'de>: std::fmt::Debug + serde::Serialize + serde::Deserialize<'de> {
  fn code(&self) -> usize {
    0
  }
}

pub fn reply<'de, T>(result: CniResult<T>) -> !
where
  T: ReplyPayload<'de>,
{
  match result {
    Ok(reply) => {
      serde_json::to_writer_pretty(std::io::stdout(), &reply).expect("Failed to serialize reply");
      std::process::exit(0);
    }
    Err(err) => {
      std::process::exit(err.code() as i32);
    }
  }
}

/// Plugins must output a JSON object with the following keys upon a successful `ADD` operation
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddReply {
  /// The same version supplied on input - the string "1.1.0"
  #[serde(
    deserialize_with = "deserialize_version",
    serialize_with = "serialize_version"
  )]
  pub cni_version: semver::Version,
  /// An array of all interfaces created by the attachment, including any host-level interfaces
  #[serde(default)]
  pub interfaces:  Vec<Interface>,
  /// IPs assigned by this attachment. Plugins may include IPs assigned external to the container.
  #[serde(default)]
  pub ips:         Vec<Ips>,
  /// Routes created by this attachment
  #[serde(default)]
  pub routes:      Vec<Route>,
  /// A dictionary consisting of DNS configuration information
  pub dns:         DNSConfig,
}

impl<'de> ReplyPayload<'de> for AddReply {}

/// [](https://github.com/containernetworking/cni/blob/main/SPEC.md#version-success)
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionReply {
  /// The value of `cniVersion` specified on input
  #[serde(
    deserialize_with = "deserialize_version",
    serialize_with = "serialize_version"
  )]
  pub cni_version:        semver::Version,
  /// A list of supported specification versions
  #[serde(
    deserialize_with = "deserialize_versions",
    serialize_with = "serialize_versions"
  )]
  pub supported_versions: Vec<Version>,
}

impl<'de> ReplyPayload<'de> for VersionReply {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interface {
  /// The name of the interface.
  pub name:    String,
  /// The hardware address of the interface (if applicable).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mac:     Option<MacAddress>,
  /// The isolation domain reference (e.g. path to network namespace) for the interface, or empty
  /// if on the host. For interfaces created inside the container, this should be the value passed
  /// via `CNI_NETNS`.
  pub sandbox: PathBuf,
}

/// IPs assigned by this attachment. Plugins may include IPs assigned external to the container.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ips {
  /// an IP address in CIDR notation (eg "192.168.1.3/24").
  pub address:   String,
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  /// the default gateway for this subnet, if one exists.
  pub gateway:   Option<IpAddr>,
  /// the index into the interfaces list for a CNI Plugin Result indicating which interface
  /// this IP configuration should be applied to.
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub interface: Option<usize>,
}

/// Routes created by this attachment
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
  /// The destination of the route, in CIDR notation
  pub dst:      IpNetwork,
  /// The next hop address. If unset, a value in `gateway` in the `ips` array may be used.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gw:       Option<IpAddr>,
  /// The MTU (Maximum transmission unit) along the path to the destination.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mtu:      Option<usize>,
  /// The MSS (Maximal Segment Size) to advertise to these destinations
  /// when establishing TCP connections.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub advmss:   Option<usize>,
  /// The priority of route, lower is higher.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub priority: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct MacAddress(pub macaddr::MacAddr);
