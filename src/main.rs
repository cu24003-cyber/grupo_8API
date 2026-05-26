use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use actix_web::{web, App, HttpServer};
mod controller;
mod models;
mod repository;
mod service;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL no encontrada");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Conectado a Supabase!");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
           .configure(controller::instructor_controller::config)
           .configure(controller::clase_controller::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .map_err(|e| sqlx::Error::Configuration(e.to_string().into()))
}