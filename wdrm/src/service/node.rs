use actix_web::{delete, get, web, HttpResponse};
use std::io::Result;
use wdrlib::zk::ZkClient;
use wdrlib::{zk_node_path, ZK_NODE_PATH};

use crate::settings::ZK_CONNECT_STRING;

#[get("/node")]
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

#[delete("/node/{node_name}")]
async fn delete_node(web::Path(node_name): web::Path<String>) -> Result<HttpResponse> {
    let zk_client = match ZkClient::new(&ZK_CONNECT_STRING) {
        Ok(zk_client) => zk_client,
        Err(err) => {
            fn_error!("Fail to connect to zk: {}", err);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    match zk_client.delete(&zk_node_path!(node_name)) {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(err) => {
            fn_error!("Fail delete node {} from zk: {}", node_name, err);
            Ok(HttpResponse::NotFound().finish())
        }
    }
}
