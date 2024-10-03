use std::str::FromStr;

use semver::Version;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

pub fn serialize_version<S>(version: &Version, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    version.to_string().serialize(serializer)
}

pub fn deserialize_version<'de, D>(deserializer: D) -> Result<Version, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let str = String::deserialize(deserializer)?;
    Version::from_str(&str).map_err(Error::custom)
}

pub fn serialize_versions<S>(versions: &[Version], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    versions
        .iter()
        .map(Version::to_string)
        .collect::<Vec<String>>()
        .serialize(serializer)
}

pub fn deserialize_versions<'de, D>(deserializer: D) -> Result<Vec<Version>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let str = Vec::<String>::deserialize(deserializer)?;
    str
        .iter()
        .map(|str| Version::from_str(str).map_err(Error::custom))
        .collect()
}
