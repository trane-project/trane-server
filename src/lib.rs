pub mod routes;
pub mod state;

use std::net;

use actix_web::{web, App, HttpServer};
use state::ServerState;

pub async fn new_trane_server<A: net::ToSocketAddrs>(address: A) -> std::io::Result<()> {
    let server_state = web::Data::new(ServerState {});

    HttpServer::new(move || {
        App::new()
            .app_data(server_state.clone())
            .service(routes::hello)
            .service(routes::echo)
    })
    .bind(address)?
    .run()
    .await
}
