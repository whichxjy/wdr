use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
struct WdrConfig {
    configs: Vec<ProcessConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
struct ProcessConfig {
    name: String,
    version: String,
}

fn main() {
    let data = r#"
        {
            "configs": [
                {
                    "name": "hello",
                    "version": "1"
                }
           ]
        }"#;

    let wdr_config: WdrConfig = serde_json::from_str(&data).unwrap();

    println!("deserialized = {:?}", wdr_config);
}
