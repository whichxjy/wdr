use serde::{Deserialize, Serialize};

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

impl WdrConfig {
    pub fn from_str(data: &str) -> Option<Self> {
        let wdr_config: WdrConfig = match serde_json::from_str(data) {
            Ok(wdr_config) => wdr_config,
            Err(err) => {
                fn_error!("Fail to parse wdr config: {}", err);
                return None;
            }
        };

        Some(wdr_config)
    }
}
