use actix_web::{App, HttpServer, middleware::Logger, web::{Data, self}};
use api::socket::grid_socket_index;
use mongo_db::MongoRepo;
use actix_cors::Cors;

mod api;
mod mongo_db;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        let cors = Cors::default().send_wildcard().allow_any_origin().allow_any_method().allow_any_header();

        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(db_data.clone())
            // add new routes here
            .service(api::grid_routes::get_grid)
            .service(api::grid_routes::post_grid)
            .route("/ws/grid/", web::get().to(grid_socket_index))
    })
    .bind(("localhost", 80))?
    .run()
    .await
}
