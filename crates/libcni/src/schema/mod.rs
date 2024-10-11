use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;
use std::str::FromStr;

use crate::schema::error::UnknownCommandError;

pub mod args;
pub mod config;
pub mod error;
pub mod ext;
pub mod reply;
pub mod runtime;

pub const DEFAULT_NET_CONF_DIR: &str = "/etc/cni/net.d";
pub const DEFAULT_NET_BIN_DIR: &str = "/opt/cni/bin";
pub const DEFAULT_NET_CACHE_DIR: &str = "/var/lib/cni";

pub type ContainerID = String;
pub type NetNS = String;
pub type IfName = String;

#[cfg(target_os = "windows")]
const PATH_SEPARATOR: char = ';';
#[cfg(not(target_os = "windows"))]
const PATH_SEPARATOR: char = ':';

#[derive(Default)]
pub struct CniPath {
  paths: Vec<PathBuf>,
}

impl FromStr for CniPath {
  type Err = std::convert::Infallible;

  fn from_str(str: &str) -> Result<Self, Self::Err> {
    let paths = str.split(PATH_SEPARATOR).map(PathBuf::from).collect();
    Ok(Self { paths })
  }
}

impl From<CniPath> for Vec<PathBuf> {
  fn from(value: CniPath) -> Self {
    value.paths
  }
}

pub enum CniCommand {
  Add,
  Del,
  Check,
  GC,
  Version,
}

impl std::error::Error for UnknownCommandError {}

impl FromStr for CniCommand {
  type Err = UnknownCommandError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "add" => Ok(CniCommand::Add),
      "del" => Ok(CniCommand::Del),
      "check" => Ok(CniCommand::Check),
      "gc" => Ok(CniCommand::GC),
      "version" => Ok(CniCommand::Version),
      _ => Err(UnknownCommandError),
    }
  }
}
