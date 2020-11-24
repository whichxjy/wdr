use gethostname::gethostname;
use std::fs;
use std::path::Path;

lazy_static! {
    // workspace path
    pub static ref WORKSPACE_PATH: &'static Path = {
        let workspace_path = Path::new("workspace");
        fs::create_dir_all(workspace_path).expect("Fail to create downloaded directory");
        workspace_path
    };
    // host name
    pub static ref HOST_NAME: String = {
        match gethostname().into_string() {
            Ok(host_name) => host_name,
            _ => "UNKNOWN_HOST".to_owned()
        }
    };
    // ip addr
    pub static ref IP_ADDR: String = {
        match local_ipaddress::get() {
            Some(ip_addr) => ip_addr,
            None => "UNKNOWN_IP_ADDR".to_owned()
        }
    };
    // zk connect string
    pub static ref ZK_CONNECT_STRING: &'static str = "localhost:2181";
}

pub fn get_wdr_node_name() -> String {
    format!("{}-{}", &HOST_NAME as &str, &IP_ADDR as &str)
}
