use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env file
    
    let _database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let database_url = String::from("postgres://todoapp:example@localhost:5432/todos"); // temporary
     
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::get_todos)
            .service(handlers::get_todo)
            .service(handlers::create_todo)
            .service(handlers::update_todo)
            .service(handlers::delete_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
