use actix_web::{HttpResponse, Responder, web};
use mongodb::{Client, Collection};
use mongodb::bson::doc;

#[path = "../../models/user.rs"]
mod model;
#[path = "../../constants/index.rs"]
mod constants;

use model::User;


pub async fn create_user(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<User> = client.database(constants::DB_NAME).collection(constants::USER_COLLECTION);
    match collection
        .find_one(doc! { "email": "ayushk@leewayhertz.com" }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_user(client: web::Data<Client>) -> mongodb::error::Result<Option<User>> {
    let collection: Collection<User> = client.database(constants::DB_NAME).collection(constants::USER_COLLECTION);
    let user_data = collection
        .find_one(doc! { "email": "ayushk@leewayhertz.com" }, None)
        .await;
    return user_data;
}

