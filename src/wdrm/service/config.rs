use actix_web::{get, HttpResponse};
use std::io::Result;
use wdrlib::config::WdrConfig;
use wdrlib::zk::ZkClient;
use wdrlib::ZK_CONFIG_PATH;

use crate::setting::ZK_CONNECT_STRING;

#[get("/config")]
async fn get_config() -> Result<HttpResponse> {
    let zk_client = match ZkClient::new(&ZK_CONNECT_STRING) {
        Ok(zk_client) => zk_client,
        Err(err) => {
            fn_error!("Fail to connect to zk: {}", err);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let data = match zk_client.get_data(&ZK_CONFIG_PATH) {
        Ok(data) => data,
        Err(err) => {
            fn_error!("Fail to read config data from zk: {}", err);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    let raw_wdr_config = match String::from_utf8(data) {
        Ok(raw_wdr_config) => raw_wdr_config,
        Err(err) => {
            fn_error!("Fail to get raw wdr config: {}", err);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    let wdr_confg = match WdrConfig::from_str(&raw_wdr_config) {
        Some(wdr_confg) => wdr_confg,
        None => {
            fn_error!("Fail to parse wdr config: {}", raw_wdr_config);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    Ok(HttpResponse::Ok().content_type("json").json(wdr_confg))
}
