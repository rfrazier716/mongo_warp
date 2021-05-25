use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use std::sync::Arc;

pub mod database;
mod routes;

pub fn launch_server(
    listener: TcpListener,
    database: database::Database,
) -> std::io::Result<Server> {
    let db = Arc::new(database);
    let server = HttpServer::new(move || {
        App::new()
            .data(db.clone())
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
