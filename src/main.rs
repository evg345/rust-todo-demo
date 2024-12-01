use actix_web::{web, App, HttpServer};
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;

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
    print!("Simple ToDo App [Rust, Postgres, REST, Swagger, Docker]\n");

    dotenv().ok(); // Load environment variables from .env file
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    print!("CONFIG: database_url={}\n", database_url);
     
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    print!("Listening at {}\n", addr);
    print!("Swagger UI - {}:/swagger-ui/\n", addr);

    HttpServer::new(move || {
        App::new()
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
