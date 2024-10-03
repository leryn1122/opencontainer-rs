use std::path::Path;

use crate::schema::error::CniResult;

pub fn load_config<P>(path: P, executable: &str) -> CniResult<()>
where
  P: AsRef<Path>,
{
  Ok(())
}
