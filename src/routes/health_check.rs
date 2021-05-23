use actix_web::{web, HttpResponse, Responder};
use mongodb::bson::doc;
use mongodb::Client;
use std::ops::Deref;
use std::sync::Arc;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn health_check_db(client: web::Data<Arc<Client>>) -> impl Responder {
    match client
        .get_ref()
        .deref()
        .database("admin")
        .collection("fruit")
        .insert_one(doc! {"color": "Red"}, None)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(x) => {
            println!("Error {:?}", x);
            HttpResponse::InternalServerError().finish()
        }
    }
}
