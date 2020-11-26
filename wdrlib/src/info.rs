use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum State {
    Init,
    Downloading,
    Ready,
    Running,
    Stopped,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct ProcessInfo {
    pub name: String,
    pub version: String,
    pub state: State,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct NodeInfo {
    pub process_info_list: Vec<ProcessInfo>,
}

impl FromStr for NodeInfo {
    type Err = serde_json::Error;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let node_info: NodeInfo = serde_json::from_str(data)?;
        Ok(node_info)
    }
}
