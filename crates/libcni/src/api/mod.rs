pub mod config;

use std::env::VarError;
use std::io::Read;
use std::str::FromStr;

use crate::schema::args::CniAddContext;
use crate::schema::args::CniArgs;
use crate::schema::args::CniCheckContext;
use crate::schema::args::CniDelContext;
use crate::schema::args::CniGcContext;
use crate::schema::args::CNI_ARGS;
use crate::schema::args::CNI_COMMAND;
use crate::schema::args::CNI_CONTAINERID;
use crate::schema::args::CNI_IFNAME;
use crate::schema::args::CNI_NSNAME;
use crate::schema::args::CNI_PATH;
use crate::schema::config::CniNetworkConfig;
use crate::schema::error::CniErrorCode;
use crate::schema::error::CniResult;
use crate::schema::CniCommand;
use crate::schema::CniPath;
use crate::schema::ContainerID;

pub trait CniPlugin {
  fn add(&self, args: CniAddContext) -> CniResult<()>;
  fn del(&self, args: CniDelContext) -> CniResult<()>;
  fn check(&self, args: CniCheckContext) -> CniResult<()>;
  fn gc(&self, args: CniGcContext) -> CniResult<()>;
  fn status(&self) -> CniResult<()>;
  fn version(&self) -> CniResult<()>;
}

pub(crate) enum CniExecution {
  Add(CniAddContext),
  Del(CniDelContext),
  Check(CniCheckContext),
  Status,
  GC(CniGcContext),
  Version,
}

impl CniExecution {
  pub fn load_args_from_env() -> CniResult<CniExecution> {
    let path = load_env::<CniPath>(CNI_PATH)?.unwrap_or_default();

    let mut buffer = Vec::with_capacity(1 << 10);
    std::io::stdin()
      .read_to_end(&mut buffer)
      .map_err(|e| CniErrorCode::IOFailure(e))?;

    let config = read_config_from_stdio()?;
    let args = obtain_args();

    match require_env::<CniCommand>(CNI_COMMAND)? {
      CniCommand::Add => Ok(Self::Add(CniAddContext {
        container_id: obtain_container_id()?,
        netns: require_env(CNI_NSNAME)?,
        if_name: require_env(CNI_IFNAME)?,
        args,
        config,
      })),
      CniCommand::Del => Ok(Self::Del(CniDelContext {
        container_id: obtain_container_id()?,
        if_name: require_env(CNI_IFNAME)?,
        netns: load_env(CNI_NSNAME)?,
        args,
        config,
      })),
      CniCommand::Check => Ok(Self::Check(CniCheckContext {
        container_id: obtain_container_id()?,
        netns: require_env(CNI_NSNAME)?,
        if_name: require_env(CNI_IFNAME)?,
        args,
        config,
      })),
      CniCommand::GC => Ok(Self::GC(CniGcContext {})),
      CniCommand::Version => Ok(Self::Version),
    }
  }

  pub fn exec(&self) -> CniResult<()> {
    Ok(())
  }
}

pub fn obtain_container_id() -> CniResult<ContainerID> {
  let container_id = require_env(CNI_CONTAINERID)?;
  check_container_id(&container_id)?;
  Ok(container_id)
}

pub fn obtain_args() -> CniArgs {
  let str = require_env::<String>(CNI_ARGS);
  if let Ok(s) = str {
    CniArgs::from_str(s.as_str()).unwrap()
  } else {
    CniArgs::default()
  }
}

pub(crate) fn check_container_id(container_id: &ContainerID) -> CniResult<()> {
  if container_id.is_empty() {
    Err(CniErrorCode::InvalidEnvironmentVariable {
      var:   CNI_CONTAINERID,
      error: Box::new(VarError::NotPresent),
    })?;
  }

  let re = regex::Regex::new(r"^[a-z0-9][a-z0-9_.\-]*$").expect("Valid container ID regex");
  if !re.is_match(container_id) {
    Err(CniErrorCode::InvalidEnvironmentVariable {
      var:   CNI_CONTAINERID,
      error: Box::new(VarError::NotPresent),
    })?;
  }
  Ok(())
}

pub fn read_config_from_stdio() -> CniResult<CniNetworkConfig> {
  let mut buffer = Vec::with_capacity(1 << 10);
  std::io::stdin()
    .read_to_end(&mut buffer)
    .map_err(|e| CniErrorCode::IOFailure(e))?;

  serde_json::from_slice::<CniNetworkConfig>(&buffer)
    .map_err(|_| CniErrorCode::InvalidNetworkConfig)
}

pub fn load_env<T>(var: &'static str) -> CniResult<Option<T>, CniErrorCode>
where
  T: FromStr,
  T::Err: std::error::Error + 'static,
{
  require_env(var).map(Some).or_else(|error| {
    if 12 == error.code() {
      Ok(None)
    } else {
      Err(error)
    }
  })
}

pub fn require_env<T>(var: &'static str) -> CniResult<T>
where
  T: FromStr,
  T::Err: std::error::Error + 'static,
{
  std::env::var(var)
    .map_err(|error| CniErrorCode::MissingEnvironmentVariable { var, error })
    .and_then(move |value| {
      value
        .parse::<T>()
        .map_err(|e| CniErrorCode::InvalidEnvironmentVariable {
          var,
          error: Box::new(e),
        })
    })
}
