use serde_json::json;

fn main() {
    let config_json = json!({
      "configs": [
        {
          "name": "hello",
          "version": "1",
          "command": []
        }
      ]
    });

    let configs = &config_json["configs"];

    println!("configs => {:#?}", configs);
}
