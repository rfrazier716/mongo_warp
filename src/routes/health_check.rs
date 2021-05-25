use crate::database;
use actix_web::{web, HttpResponse, Responder};
use std::ops::Deref;
use std::sync::Arc;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn health_check_db(database: web::Data<Arc<database::Database>>) -> impl Responder {
    // match client
    match database.get_ref().deref().ping().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(x) => {
            println!("Error {:?}", x);
            HttpResponse::InternalServerError().finish()
        }
    }
}
