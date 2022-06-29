use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};

#[get("/test")]
pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("I'm wow!")
}

async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}