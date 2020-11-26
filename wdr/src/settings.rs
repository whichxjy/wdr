use gethostname::gethostname;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

lazy_static! {
    // settings
    pub static ref SETTINGS_MAP: HashMap<String, String> = {
        let mut settings = config::Config::default();
        settings.set_default("workspace", "workspace").unwrap();
        settings.set_default("zk_connect_string", "localhost:2181").unwrap();

        // Merge file setting.
        if let Err(err) = settings.merge(config::File::with_name("Settings")) {
            fn_warn!("Fail to read settings from file: {}", err);
        }
        // Merge env setting.
        if let Err(err) = settings.merge(config::Environment::with_prefix("WDR")) {
            fn_warn!("Fail to read settings from env: {}", err);
        }

        settings
            .try_into::<HashMap<String, String>>()
            .expect("No settings provided")
    };
    // workspace path
    pub static ref WORKSPACE_PATH: &'static Path = {
        let workspace_setting = SETTINGS_MAP.get("workspace").unwrap();
        let workspace_path = Path::new(workspace_setting);
        fs::create_dir_all(workspace_path).expect("Fail to create downloaded directory");
        workspace_path
    };
    // zk connect string
    pub static ref ZK_CONNECT_STRING: &'static str = {
        SETTINGS_MAP.get("zk_connect_string").unwrap()
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
}

pub fn init() {
    let settings_map = &SETTINGS_MAP as &HashMap<String, String>;
    fn_debug!("Settings: {:#?}", settings_map);
}

pub fn get_wdr_node_name() -> String {
    format!("{}-{}", &HOST_NAME as &str, &IP_ADDR as &str)
}
