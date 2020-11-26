use actix_web::{get, post, web, HttpResponse};
use std::io::Result;
use wdrlib::config::WdrConfig;
use wdrlib::zk::{CreateMode, ZkClient};
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

    let raw_data = match zk_client.get_data(&ZK_CONFIG_PATH) {
        Ok(raw_data) => raw_data,
        Err(err) => {
            fn_error!("Fail to get raw data from zk: {}", err);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    let data = match String::from_utf8(raw_data) {
        Ok(data) => data,
        Err(err) => {
            fn_error!("Fail to convert raw data: {}", err);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    let wdr_confg = match WdrConfig::from_str(&data) {
        Some(wdr_confg) => wdr_confg,
        None => {
            fn_error!("Fail to parse wdr config: {}", data);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(wdr_confg))
}

#[post("/config")]
async fn set_config(wdr_confg: web::Json<WdrConfig>) -> Result<HttpResponse> {
    let zk_client = match ZkClient::new(&ZK_CONNECT_STRING) {
        Ok(zk_client) => zk_client,
        Err(err) => {
            fn_error!("Fail to connect to zk: {}", err);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let data = serde_json::to_string(&wdr_confg.into_inner()).unwrap();

    if let Err(err) = zk_client.set_data(
        &ZK_CONFIG_PATH,
        data.as_bytes().to_vec(),
        CreateMode::Persistent,
    ) {
        fn_error!("Fail to write wdr config: {}", err);
        return Ok(HttpResponse::InternalServerError().finish());
    }

    Ok(HttpResponse::Ok().finish())
}
