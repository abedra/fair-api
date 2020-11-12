mod types;
mod heartbeat;
mod scenario;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .configure(heartbeat::init_routes)
        .configure(scenario::init_routes))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
