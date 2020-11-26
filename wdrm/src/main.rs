#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate fnlog;

mod service;
mod settings;

use actix_web::{App, HttpServer};
use settings::ADDRESS;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    settings::init();
    HttpServer::new(|| {
        App::new()
            .service(service::config::get_config)
            .service(service::config::set_config)
            .service(service::node::get_node_list)
            .service(service::node::delete_node)
            .service(service::info::get_node_info)
    })
    .bind(&ADDRESS as &str)?
    .run()
    .await
}
