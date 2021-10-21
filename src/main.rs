#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod types;
mod heartbeat;
mod scenario;
mod schema;
mod db;
mod error_handler;

use dotenv::dotenv;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    HttpServer::new(|| App::new()
        .configure(heartbeat::init_routes)
        .configure(scenario::init_routes))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
