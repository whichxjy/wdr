#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate fnlog;

mod setting;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use wdrlib::config::WdrConfig;
use wdrlib::zk::ZkClient;
use wdrlib::ZK_CONFIG_PATH;

use crate::setting::ZK_CONNECT_STRING;

#[get("/config")]
async fn get_config() -> impl Responder {
    let zk_client = ZkClient::new(&ZK_CONNECT_STRING).expect("Fail to connect to zk");

    let data = zk_client.get_data(&ZK_CONFIG_PATH).unwrap();
    let raw_config = String::from_utf8(data).unwrap();
    let confg = WdrConfig::from_str(&raw_config);
    let res = serde_json::to_string(&confg).unwrap();

    HttpResponse::Ok().body(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| App::new().service(get_config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
