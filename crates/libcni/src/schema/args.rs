/*
[Parameters](https://github.com/containernetworking/cni/blob/main/SPEC.md#parameters)
 */
use std::collections::HashMap;
use std::str::FromStr;

/// `CNI_COMMAND`: indicates the desired operation; `ADD`, `DEL`, `CHECK`, `GC`, or `VERSION`.
pub const CNI_COMMAND: &str = "CNI_COMMAND";
/// Container ID. A unique plaintext identifier for a container, allocated by the runtime.
pub const CNI_CONTAINERID: &str = "CNI_CONTAINERID";
/// A reference to the container's "isolation domain".
pub const CNI_NSNAME: &str = "CNI_NSNAME";
/// Name of the interface to create inside the container.
pub const CNI_IFNAME: &str = "CNI_IFNAME";
/// Extra arguments passed in by the user at invocation time.
pub const CNI_ARGS: &str = "CNI_ARGS";
/// List of paths to search for CNI plugin executables.
pub const CNI_PATH: &str = "CNI_PATH";

#[derive(Default)]
pub struct CniArgs {
  args: HashMap<String, String>,
}

impl FromStr for CniArgs {
  type Err = std::convert::Infallible;

  fn from_str(str: &str) -> Result<Self, Self::Err> {
    let mut args = HashMap::new();

    str.split(';').filter(|s| !s.is_empty()).for_each(|entry| {
      let parts: Vec<String> = entry.splitn(2, '=').map(|s| s.to_string()).collect();
      args.insert(parts[0].to_string(), parts[1].to_string());
    });
    Ok(Self { args })
  }
}

impl From<CniArgs> for HashMap<String, String> {
  fn from(value: CniArgs) -> Self {
    value.args
  }
}
