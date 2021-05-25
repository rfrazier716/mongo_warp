use crate::database;
use crate::error::ServerError;
use actix_web::{web, HttpResponse, Responder};
use std::ops::Deref;
use std::sync::Arc;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn health_check_db(
    database: web::Data<Arc<database::Database>>,
) -> Result<HttpResponse, ServerError> {
    // match client
    match database.get_ref().deref().ping().await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(x) => {
            println!("{:?}", x);
            Err(ServerError::DataBaseError { source: x })
        }
    }
}
