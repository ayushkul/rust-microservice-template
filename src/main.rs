use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
#[path = "app/modules/test/index.rs"] mod create_user;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rust service prototype")
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::scope("/routes"))
            .service(index)
            .service(healthcheck)
            .service(create_user::create_user)
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(init)
    })
        .bind(("127.0.0.1", 3001))?
        .run()
        .await
}
