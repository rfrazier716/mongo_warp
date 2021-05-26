use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use std::sync::Arc;

pub mod database;
pub mod error;
pub mod routes;
pub mod startup;
