mod handlers;
mod models;

use axum::{routing::get, Router};
use dotenv::dotenv;
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use axum::routing::{delete, post, put};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::list_todos,
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

#[tokio::main]
async fn main() {
    print!("=== Simple ToDo App [Rust, Postgres, REST, Swagger, Docker] ===\n");

    dotenv().ok(); // Load environment variables from .env file

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    info!("CONFIG: database_url={}", database_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let app = Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route("/todos", get(handlers::list_todos))
        .route("/todos/:id", get(handlers::get_todo))
        .route("/todos", post(handlers::create_todo))
        .route("/todos/:id", put(handlers::update_todo))
        .route("/todos/:id", delete(handlers::delete_todo))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(pool)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    info!("Listening at {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("Swagger UI - {}:/swagger-ui/", addr);
    axum::serve(listener, app).await.unwrap();
}
