use actix_web::{get, web, HttpResponse};
use std::io::Result;
use std::str::FromStr;
use wdrlib::info::NodeInfo;
use wdrlib::zk::ZkClient;
use wdrlib::zk_node_info_path;

use crate::setting::ZK_CONNECT_STRING;

#[get("/info/{node_name}")]
async fn get_node_info(web::Path(node_name): web::Path<String>) -> Result<HttpResponse> {
    let zk_client = match ZkClient::new(&ZK_CONNECT_STRING) {
        Ok(zk_client) => zk_client,
        Err(err) => {
            fn_error!("Fail to connect to zk: {}", err);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let raw_data = match zk_client.get_data(&zk_node_info_path!(node_name)) {
        Ok(data) => data,
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

    let node_info: NodeInfo = match NodeInfo::from_str(&data) {
        Ok(node_info) => node_info,
        Err(err) => {
            fn_error!("Fail to parse raw node info: {}", err);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(node_info))
}
