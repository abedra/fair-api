use actix_web::{web, get, Responder, HttpResponse};
use crate::heartbeat::model::generate_heartbeat;

#[get("/")]
async fn heartbeat() -> impl Responder {
    let result = serde_json::to_string(&generate_heartbeat());
    match result {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(heartbeat);
}