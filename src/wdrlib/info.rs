use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum State {
    Init,
    Downloading,
    Ready,
    Running,
    Stopped,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct ProcessInfo {
    pub name: String,
    pub version: String,
    pub state: State,
}
