use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct WdrConfig {
    configs: Vec<ProcessConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
struct ProcessConfig {
    name: String,
    version: String,
}

impl WdrConfig {
    pub fn from_str(data: &str) -> Result<Self, ()> {
        let wdr_config: WdrConfig = match serde_json::from_str(data) {
            Ok(wdr_config) => wdr_config,
            _ => return Err(()),
        };

        Ok(wdr_config)
    }
}