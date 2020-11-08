mod fair;
mod types;
mod heartbeater;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::types::Scenario;
use crate::fair::model_scenario;
use crate::heartbeater::generate_heartbeat;

#[get("/")]
async fn heartbeat() -> impl Responder {
    let result = serde_json::to_string(&generate_heartbeat());
    match result {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[post("/")]
async fn model(item: web::Json<Scenario>) -> impl Responder {
    let result = serde_json::to_string(&model_scenario(&item.0));
    match result {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(heartbeat)
        .service(model))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
