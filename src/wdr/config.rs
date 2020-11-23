use std::fs;
use std::path::Path;

lazy_static! {
    // workspace path
    pub static ref WORKSPACE_PATH: &'static Path = {
        let workspace_path = Path::new("workspace");
        fs::create_dir_all(workspace_path).expect("Fail to create downloaded directory");
        workspace_path
    };
    // zk connect string
    pub static ref ZK_CONNECT_STRING: &'static str = "localhost:2181";
    // zk config path
    pub static ref ZK_CONFIG_PATH: &'static str = "/config";
}
