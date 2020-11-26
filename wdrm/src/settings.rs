use std::collections::HashMap;

lazy_static! {
    // settings
    pub static ref SETTINGS_MAP: HashMap<String, String> = {
        let mut settings = config::Config::default();
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
    // zk connect string
    pub static ref ZK_CONNECT_STRING: &'static str = {
        SETTINGS_MAP.get("zk_connect_string").unwrap()
    };
}

pub fn init() {
    let settings_map = &SETTINGS_MAP as &HashMap<String, String>;
    fn_debug!("Settings: {:#?}", settings_map);
}
