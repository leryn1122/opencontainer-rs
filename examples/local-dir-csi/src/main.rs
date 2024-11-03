use std::path::PathBuf;

use clap::Parser;
use clap_derive::Parser;
use tokio::net::UnixListener;
use tonic::codegen::tokio_stream::wrappers::UnixListenerStream;

use crate::driver::ControllerServerImpl;
use crate::driver::IdentityServerImpl;
use crate::driver::NodeServerImpl;

pub mod driver;
use crate::log_ext::LogLevelParser;

#[derive(Parser)]
pub struct Flags {
  #[clap(long = "endpoint", default_value = "unix:///csi/csi.sock")]
  endpoint:    PathBuf,
  #[clap(long = "nodeid")]
  node_id:     String,
  #[clap(long = "drivername")]
  driver_name: String,
  #[clap(long = "v", default_value_t = log::Level::Info, value_parser = LogLevelParser::default())]
  log_level:   log::Level,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Flags::parse();

  let _ = std::fs::remove_file(args.endpoint.clone());
  let uds = UnixListener::bind(args.endpoint.clone())?;
  let uds_stream = UnixListenerStream::new(uds);

  let layer = tower::ServiceBuilder::new().into_inner();
  let mut builder = tonic::transport::Server::builder().layer(layer);

  builder
    .add_service(libcsi::v1::identity_server::IdentityServer::new(
      IdentityServerImpl::new(""),
    ))
    .add_service(libcsi::v1::node_server::NodeServer::new(
      NodeServerImpl::new(args.node_id.clone()),
    ))
    .add_service(libcsi::v1::controller_server::ControllerServer::new(
      ControllerServerImpl::new(""),
    ))
    .serve_with_incoming(uds_stream)
    .await?;

  Ok(())
}

pub mod log_ext {
  use std::ffi::OsStr;
  use std::ffi::OsString;

  use opentelemetry_appender_log::OpenTelemetryLogBridge;
  use opentelemetry_sdk::logs::BatchLogProcessor;
  use opentelemetry_sdk::logs::LoggerProvider;
  use opentelemetry_sdk::runtime;
  use opentelemetry_stdout::LogExporter;

  pub fn init_logger(log_level: log::Level) {
    let exporter = LogExporter::default();
    let logger_provider = LoggerProvider::builder()
      .with_log_processor(BatchLogProcessor::builder(exporter, runtime::Tokio).build())
      .build();
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
    log::set_max_level(log_level.to_level_filter());
  }

  #[derive(Clone, Default)]
  pub struct LogLevelParser;

  impl clap::builder::TypedValueParser for LogLevelParser {
    type Value = log::Level;

    fn parse_ref(
      &self,
      cmd: &clap::Command,
      arg: Option<&clap::Arg>,
      value: &OsStr,
    ) -> Result<Self::Value, clap::Error> {
      self.parse(cmd, arg, value.to_owned())
    }

    fn parse(
      &self,
      cmd: &clap::Command,
      _arg: Option<&clap::Arg>,
      value: OsString,
    ) -> Result<Self::Value, clap::Error> {
      value
        .into_string()
        .map_err(|_| clap::Error::new(clap::error::ErrorKind::InvalidUtf8).with_cmd(cmd))
        .map(|s| s.parse::<usize>().unwrap_or(0))
        .map(|u| from_usize(u))
        .map(|l| l.unwrap_or(log::Level::Error))
        .map_err(|e| clap::Error::new(clap::error::ErrorKind::InvalidUtf8).with_cmd(cmd))
    }
  }

  fn from_usize(u: usize) -> Option<log::Level> {
    match u {
      1 => Some(log::Level::Error),
      2 => Some(log::Level::Warn),
      3 => Some(log::Level::Info),
      4 => Some(log::Level::Debug),
      5 => Some(log::Level::Trace),
      _ => None,
    }
  }
}
