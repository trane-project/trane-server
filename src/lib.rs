pub mod config;
pub mod error;
pub mod routes;
pub mod server;

use actix_web::{web, App, HttpServer};
use config::ServerConfig;
use server::Server;

pub async fn new_trane_server(config: ServerConfig) -> std::io::Result<()> {
    let server_state = web::Data::new(Server {
        config: config.clone(),
        remote_instances: Default::default(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(server_state.clone())
            .service(routes::open_library)
            .service(routes::close_library)
            .service(routes::apply_filter)
            .service(routes::ffi::apply_filter)
            .service(routes::get_exercise_batch)
            .service(routes::ffi::get_exercise_batch)
            .service(routes::healthz)
    })
    .bind((config.address, config.port))?
    .run()
    .await
}
