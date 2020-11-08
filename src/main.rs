mod fair;
mod types;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::types::Scenario;
use crate::fair::model_scenario;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("OK")
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
    HttpServer::new(|| App::new().service(hello).service(model))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
