use actix::Actor;
use actix_web::{App, HttpServer, middleware::Logger, web::{Data, self}};
use api::socket::{grid_socket_index};
use actix_cors::Cors;
use actors::socket_data::SocketData;

mod api;
mod mongo_db;
mod models;
mod actors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let ws_data = Data::new(SocketData::new().start());

    HttpServer::new(move || {
        let cors = Cors::default().send_wildcard().allow_any_origin().allow_any_method().allow_any_header();

        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(ws_data.clone())
            // add new routes here
            .route("/ws/grid/", web::get().to(grid_socket_index))
    })
    .bind(("localhost", 80))?
    .run()
    .await
}
