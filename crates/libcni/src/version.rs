use crate::schema::error::CniResult;

pub fn check_version(version: &semver::Version, supported: Vec<semver::Version>) -> CniResult<()> {
  // TODO: Unimplemented
  Ok(())
}

pub fn current() -> semver::Version {
  semver::Version::parse("1.1.0").unwrap()
}

pub trait PluginInfo {
  fn supported_versions(&self) -> Vec<semver::Version>;
}

pub struct Legacy;

impl PluginInfo for Legacy {
  fn supported_versions(&self) -> Vec<semver::Version> {
    Vec::from([
      semver::Version::parse("0.1.0").unwrap(),
      semver::Version::parse("0.2.0").unwrap(),
    ])
  }
}

pub struct All;

impl PluginInfo for All {
  fn supported_versions(&self) -> Vec<semver::Version> {
    Vec::from([
      semver::Version::parse("0.1.0").unwrap(),
      semver::Version::parse("0.2.0").unwrap(),
      semver::Version::parse("0.3.0").unwrap(),
      semver::Version::parse("0.3.1").unwrap(),
      semver::Version::parse("0.4.0").unwrap(),
      semver::Version::parse("1.0.0").unwrap(),
      semver::Version::parse("1.1.0").unwrap(),
    ])
  }
}
