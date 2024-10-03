use crate::schema::args::CniArgs;
use crate::schema::ContainerID;
use crate::schema::IfName;
use crate::schema::NetNS;

pub struct RuntimeConfig {
  pub container_id: ContainerID,
  pub net_ns:       NetNS,
  pub if_name:      IfName,
  pub args:         CniArgs,
}
