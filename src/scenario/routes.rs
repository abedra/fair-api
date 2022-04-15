use actix_web::{web, post, Responder, HttpResponse};
use crate::types::Scenario;
use crate::scenario::model::model_scenario;

#[post("/")]
async fn model(item: web::Json<Scenario>) -> impl Responder {
    match model_scenario(&item.0) {
        Err(e) => {
            HttpResponse::Ok().body(serde_json::to_string(&e).unwrap())
        },
        Ok(result) => {
            HttpResponse::Ok().body(serde_json::to_string(&result).unwrap())
        }
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(model);
}
