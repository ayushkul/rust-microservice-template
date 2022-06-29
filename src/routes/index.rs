use actix_web::{get, post, HttpResponse, Responder, web};
use mongodb::Client;

#[path = "../app/modules/user/index.rs"]
mod user_controller;


#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rust microservice alive!")
}

#[get("/user")]
pub async fn get_user(client: web::Data<Client>) -> impl Responder {
    let user_details = user_controller::get_user(client).await;
    HttpResponse::Ok().json(user_details)
}


#[post("/user")]
pub async fn create_user(client: web::Data<Client>) -> impl Responder {
    let user_details = user_controller::create_user(client).await;
    HttpResponse::Ok().body("user_details")
}

