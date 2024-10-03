pub mod config;

use crate::schema::error::CniResult;

pub trait CniPlugin {
  fn add(&self) -> CniResult<()>;
  fn del(&self) -> CniResult<()>;
  fn version(&self) -> CniResult<()>;
  fn gc(&self) -> CniResult<()>;
  fn status(&self) -> CniResult<()>;
}
