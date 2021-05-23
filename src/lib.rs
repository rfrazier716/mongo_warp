use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use mongodb::Client;
use std::net::TcpListener;
use std::sync::Arc;

mod routes;

pub fn launch_server(listener: TcpListener, client: Client) -> std::io::Result<Server> {
    let client = Arc::new(client);
    let server = HttpServer::new(move || {
        App::new()
            .data(client.clone())
            .route(
                "/health_check",
                web::get().to(routes::health_check::health_check),
            )
            .route(
                "/health_check/db",
                web::get().to(routes::health_check::health_check_db),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
