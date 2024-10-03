use std::borrow::Cow;
use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;
use support::semver::deserialize_version;
use support::semver::serialize_version;

///
pub type CniResult<T, E = CniError> = Result<T, E>;

/// # CNI Error
/// [Error](https://github.com/containernetworking/cni/blob/main/SPEC.md#error)
#[derive(Debug, Deserialize, Serialize)]
pub struct CniError {
  /// The protocol version in use - "1.1.0"
  #[serde(
    deserialize_with = "deserialize_version",
    serialize_with = "serialize_version"
  )]
  pub cni_version: semver::Version,
  /// A numeric error code.
  pub code:        usize,
  /// A short message characterizing the error.
  #[serde(rename = "msg")]
  pub message:     Cow<'static, str>,
  /// A longer message describing the error.
  #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
  pub details:     Option<Cow<'static, str>>,
}

impl CniError {
  pub fn new(code: usize, message: impl Into<String>, details: Option<impl Into<String>>) -> Self {
    Self {
      cni_version: semver::Version::new(1, 1, 0),
      code,
      message: Cow::from(message.into()),
      details: details.map(|s| Cow::from(s.into())),
    }
  }

  pub fn simple(code: usize, message: impl Into<String>) -> Self {
    Self::new(code, message.into(), None::<&str>)
  }
}

impl Display for CniError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.message.as_ref())
  }
}

impl std::error::Error for CniError {}

/// # CNI Error Code
/// [Error Code](https://github.com/containernetworking/cni/blob/main/SPEC.md#well-known-error-codes)
pub enum CniErrorCode {
  /// Incompatible CNI version
  IncompatibleVersion(semver::Version),
  /// Unsupported field in network configuration.
  /// The error message must contain the key and value of the unsupported field.
  UnsupportedField,
  /// Container unknown or does not exist.
  /// This error implies the runtime does not need to perform any container network cleanup
  /// (for example, calling the DEL action on the container).
  UnknownContainer,
  /// Invalid necessary environment variables, like `CNI_COMMAND`, `CNI_CONTAINERID`, etc.
  /// The error message must contain the names of invalid variables.
  InvalidEnvironmentVariable {
    var:   &'static str,
    error: Box<dyn std::error::Error>,
  },
  /// I/O failure. For example, failed to read network config bytes from stdin.
  IOFailure,
  /// Failed to decode content. For example, failed to unmarshal network config from bytes or
  /// failed to decode version info from string.
  DecodeContentFailure,
  /// Invalid network config. If some validations on network configs do not pass,
  /// this error will be raised.
  InvalidNetworkConfig,
  /// Try again later. If the plugin detects some transient condition that should clear up,
  /// it can use this code to notify the runtime it should re-try the operation later.
  TryAgainLater,
}

impl Into<usize> for CniErrorCode {
  fn into(self) -> usize {
    match self {
      CniErrorCode::IncompatibleVersion(_) => 1,
      CniErrorCode::UnsupportedField => 2,
      CniErrorCode::UnknownContainer => 3,
      CniErrorCode::InvalidEnvironmentVariable { .. } => 4,
      CniErrorCode::IOFailure => 5,
      CniErrorCode::DecodeContentFailure => 6,
      CniErrorCode::InvalidNetworkConfig => 7,
      CniErrorCode::TryAgainLater => 11,
    }
  }
}