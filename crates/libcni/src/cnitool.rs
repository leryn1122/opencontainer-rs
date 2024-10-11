#![feature(io_error_more)]

pub mod api;
pub mod schema;

use std::borrow::Cow;
use std::ops::Index;
use std::path::PathBuf;
use std::str::FromStr;

use libcni::api::config::load_config;
use libcni::schema::args::CniArgs;
use libcni::schema::args::CNI_ARGS;
use libcni::schema::args::CNI_IFNAME;
use libcni::schema::error::CniError;
use libcni::schema::error::CniErrorCode;
use libcni::schema::error::CniResult;
use libcni::schema::runtime::RuntimeConfig;
use libcni::schema::ContainerID;
use libcni::schema::DEFAULT_NET_CONF_DIR;
use sha2::Digest;
use sha2::Sha512;

fn main() {
  run().unwrap_or_else(|e| eprintln!("{:?}", e));
}

fn run() -> CniResult<()> {
  if std::env::args().len() < 4 {
    print_usage();
  }

  let args = std::env::args().collect::<Vec<String>>();

  let net_dir = std::env::var("NETCONFPATH")
    .ok()
    .or_else(|| Some(DEFAULT_NET_CONF_DIR.to_string()))
    .expect("CNI network configuration directory is set");

  let _ = load_config(PathBuf::from(net_dir), args.index(2))
    .map_err(|_| exit(Some(CniErrorCode::DecodeContentFailure)))
    .expect("Failed to load config file");

  let if_name = std::env::var(CNI_IFNAME)
    .ok()
    .or_else(|| Some("eth0".to_string()))
    .expect("CNI_IFNAME env var is not set");

  let netns = args.index(3);
  if !PathBuf::from(netns).is_absolute() {
    exit(Some(CniErrorCode::IOFailure(std::io::Error::new(
      std::io::ErrorKind::InvalidFilename,
      "No network configuration path must be absolute",
    ))))
  }

  let cni_args = std::env::var(CNI_ARGS)
    .ok()
    .map(|s| CniArgs::from_str(&s).expect("Failed to parse arguments from CNI env"))
    .expect("CNI arguments is empty");

  let rt = RuntimeConfig {
    container_id: obtain_hashed_container_id(netns.clone()),
    netns: netns.clone(),
    if_name,
    args: cni_args,
  };

  match args.index(1).as_str() {
    "add" => exit(None),
    "del" => exit(None),
    "check" => exit(None),
    "gc" => exit(None),
    "status" => exit(None),
    _ => exit(Some(CniErrorCode::UnknownCommand)),
  }
}

fn print_usage() {
  eprintln!(
    "\
cnitool: Add, check, or remove network interfaces from a network namespace
  cnitool add   <net> <netns>
  cnitool check <net> <netns>
  cnitool del   <net> <netns>"
  );
  std::process::exit(1);
}

fn exit(error: Option<CniErrorCode>) -> ! {
  if let Some(e) = error {
    eprintln!("{:?}", e);
    std::process::exit(1);
  } else {
    std::process::exit(0);
  }
}

fn obtain_hashed_container_id(content: String) -> ContainerID {
  let mut hasher = Sha512::new();
  hasher.update(format!("cnitool-{:10}", content));
  let bytes = &hasher.finalize()[..];
  String::from_utf8_lossy(bytes).to_string()
}
