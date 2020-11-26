use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct WdrConfig {
    pub configs: Vec<ProcessConfig>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct ProcessConfig {
    pub name: String,
    pub version: String,
    pub resource: String,
    pub cmd: String,
}

impl FromStr for WdrConfig {
    type Err = serde_json::Error;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let wdr_config: WdrConfig = serde_json::from_str(data)?;
        Ok(wdr_config)
    }
}
