use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct WdrConfig {
    pub configs: Vec<ProcessConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct ProcessConfig {
    pub name: String,
    pub version: String,
    pub resource: String,
}

impl WdrConfig {
    pub fn from_str(data: &str) -> Option<Self> {
        let wdr_config: WdrConfig = match serde_json::from_str(data) {
            Ok(wdr_config) => wdr_config,
            Err(err) => {
                wdr_error!("{}", err);
                return None;
            }
        };

        Some(wdr_config)
    }
}
