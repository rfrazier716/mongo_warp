use zero_to_prod::launch_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    launch_server()?.await // start the server as an async function
}
