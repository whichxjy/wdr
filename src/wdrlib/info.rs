use serde::{Deserialize, Serialize};

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

impl NodeInfo {
    pub fn from_str(data: &str) -> Option<Self> {
        let node_info: NodeInfo = match serde_json::from_str(data) {
            Ok(node_info) => node_info,
            Err(err) => {
                fn_error!("Fail to parse node info: {}", err);
                return None;
            }
        };

        Some(node_info)
    }
}
