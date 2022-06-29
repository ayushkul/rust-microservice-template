use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rust service prototype")
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}


#[get("/test")]
pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("I'm wow!")
}

