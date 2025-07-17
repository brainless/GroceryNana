use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;

#[derive(Debug, Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct HelloResponse {
    pub message: String,
}

async fn health_check(db: web::Data<Pool<Sqlite>>) -> Result<HttpResponse> {
    // Test database connection
    match sqlx::query("SELECT 1").fetch_one(db.as_ref()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(HealthResponse {
            status: "ok".to_string(),
            message: "Database connected".to_string(),
        })),
        Err(_) => Ok(HttpResponse::InternalServerError().json(HealthResponse {
            status: "error".to_string(),
            message: "Database connection failed".to_string(),
        })),
    }
}

async fn hello_world() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(HelloResponse {
        message: "Hello World from GroceryNana Backend!".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment variables
    dotenv::dotenv().ok();
    env_logger::init();

    // Database setup
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./database.db".to_string());

    // Create database pool
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to create database pool");

    // Create database if it doesn't exist
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    log::info!("Starting GroceryNana Backend server on http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .route("/", web::get().to(hello_world))
            .route("/api/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
