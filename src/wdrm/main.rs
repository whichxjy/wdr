#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate fnlog;

mod setting;

use actix_web::{get, App, HttpResponse, HttpServer};
use std::io::Result;
use wdrlib::config::WdrConfig;
use wdrlib::zk::ZkClient;
use wdrlib::ZK_CONFIG_PATH;

use crate::setting::ZK_CONNECT_STRING;

#[get("/config")]
async fn get_config() -> Result<HttpResponse> {
    let zk_client = ZkClient::new(&ZK_CONNECT_STRING).expect("Fail to connect to zk");

    let data = zk_client.get_data(&ZK_CONFIG_PATH).unwrap();
    let raw_wdr_config = String::from_utf8(data).unwrap();
    let wdr_confg = WdrConfig::from_str(&raw_wdr_config).unwrap();

    Ok(HttpResponse::Ok().json(wdr_confg))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| App::new().service(get_config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
