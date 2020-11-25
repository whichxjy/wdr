use actix_web::{get, web, HttpResponse};
use std::io::Result;
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

    let data = match zk_client.get_data(&zk_node_info_path!(node_name)) {
        Ok(data) => data,
        Err(err) => {
            fn_error!("Fail to read info data from zk: {}", err);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    let raw_node_info = match String::from_utf8(data) {
        Ok(raw_node_info) => raw_node_info,
        Err(err) => {
            fn_error!("Fail to get raw node info: {}", err);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    let node_info = match NodeInfo::from_str(&raw_node_info) {
        Some(node_info) => node_info,
        None => {
            fn_error!("Fail to parse raw node info: {}", raw_node_info);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(node_info))
}
