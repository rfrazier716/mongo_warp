use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn launch_server() -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(("127.0.0.1", 8080))?
        .run();
    Ok(server)
}
