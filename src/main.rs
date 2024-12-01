use actix_web::{middleware, web, App, HttpServer};
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use log::info;

mod handlers;

mod models;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::get_todos,
        handlers::get_todo,
        handlers::create_todo,
        handlers::update_todo,
        handlers::delete_todo
    ),
    components(
        schemas(
            models::TodoRec,
            models::CreateTodoRequest,
            models::UpdateTodoRequest
        )
    ),
    tags((name = "BasicToDoREST", description = "A very Basic REST API for simple ToDo App"))
)]
pub struct ApiDoc;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print!("=== Simple ToDo App [Rust, Postgres, REST, Swagger, Docker] ===\n");

    dotenv().ok(); // Load environment variables from .env file
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    info!("CONFIG: database_url={}", database_url);
     
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    info!("Listening at {}", addr);
    info!("Swagger UI - {}:/swagger-ui/", addr);

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // Shared state - DB Connection pool
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::get_todos)
            .service(handlers::get_todo)
            .service(handlers::create_todo)
            .service(handlers::update_todo)
            .service(handlers::delete_todo)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(addr)?
    .run()
    .await
}
