use actix_web::{delete, get, web, HttpResponse};
use std::io::Result;
use std::str::FromStr;
use wdrlib::info::NodeInfo;
use wdrlib::zk::ZkClient;
use wdrlib::zk_node_info_path;
use wdrlib::{zk_node_path, ZK_NODE_PATH};

use crate::settings::ZK_CONNECT_STRING;

#[get("/nodes")]
async fn get_node_list() -> Result<HttpResponse> {
    let zk_client = match ZkClient::new(&ZK_CONNECT_STRING) {
        Ok(zk_client) => zk_client,
        Err(err) => {
            fn_error!("Fail to connect to zk: {}", err);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let children = match zk_client.get_children(&ZK_NODE_PATH) {
        Ok(children) => children,
        Err(err) => {
            fn_error!("Fail get children from zk: {}", err);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(children))
}

#[get("/nodes/{node_name}/info")]
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

#[delete("/nodes/{node_name}")]
async fn delete_node(web::Path(node_name): web::Path<String>) -> Result<HttpResponse> {
    let zk_client = match ZkClient::new(&ZK_CONNECT_STRING) {
        Ok(zk_client) => zk_client,
        Err(err) => {
            fn_error!("Fail to connect to zk: {}", err);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let _ = zk_client.delete(&zk_node_path!(node_name));
    Ok(HttpResponse::Ok().finish())
}
