use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

mod routes;

pub fn launch_server(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new().route(
            "/health_check",
            web::get().to(routes::health_check::health_check),
        )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
