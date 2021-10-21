use actix_web::{web, post, Responder, HttpResponse};
use crate::scenario::fair::model_scenario;
use either::Either;
use crate::scenario::model::Scenario;

#[post("/")]
async fn model(item: web::Json<Scenario>) -> impl Responder {
    match model_scenario(&item.0) {
        Either::Left(e) => {
            HttpResponse::Ok().body(serde_json::to_string(&e).unwrap())
        },
        Either::Right(result) => {
            HttpResponse::Ok().body(serde_json::to_string(&result).unwrap())
        }
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(model);
}
