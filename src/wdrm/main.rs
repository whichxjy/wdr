#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate fnlog;

mod service;
mod setting;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .service(service::config::get_config)
            .service(service::info::get_node_info)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
