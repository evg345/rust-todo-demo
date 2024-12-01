use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use tracing::{info, error, instrument};
use crate::models::{CreateTodoRequest, TodoRec, UpdateTodoRequest};

/// List all todos
#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "List all todos successfully", body = Vec<TodoRec>),
        (status = 500, description = "Internal server error")
    ),
    tag = "todos"
)]
#[instrument(skip(pool))]
pub async fn list_todos(
    State(pool): State<PgPool>
) -> Result<Json<Vec<TodoRec>>, StatusCode> {
    match sqlx::query_as!(
        TodoRec,
        "SELECT * FROM todos WHERE user_id = $1 ORDER BY created_at DESC",
        1 // Hardcoded user_id for now
    )
    .fetch_all(&pool)
    .await
    {
        Ok(todos) => {
            info!("Retrieved {} todos", todos.len());
            Ok(Json(todos))
        }
        Err(e) => {
            error!("Database error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get a specific todo by ID
#[utoipa::path(
    get,
    path = "/todos/{id}",
    responses(
        (status = 200, description = "Todo found successfully", body = TodoRec),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "Todo ID")
    ),
    tag = "todos"
)]
#[instrument(skip(pool))]
pub async fn get_todo(
    State(pool): State<PgPool>,
    Path(todo_id): Path<i32>,
) -> Result<Json<TodoRec>, StatusCode> {
    match sqlx::query_as!(
        TodoRec,
        "SELECT * FROM todos WHERE todo_id = $1 AND user_id = $2",
        todo_id,
        1 // Hardcoded user_id for now
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(todo)) => {
            info!("Retrieved todo {}", todo_id);
            Ok(Json(todo))
        }
        Ok(None) => {
            info!("Todo {} not found", todo_id);
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("Database error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create a new todo
#[utoipa::path(
    post,
    path = "/todos",
    request_body = CreateTodoRequest,
    responses(
        (status = 201, description = "Todo created successfully", body = TodoRec),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "todos"
)]
#[instrument(skip(pool))]
pub async fn create_todo(
    State(pool): State<PgPool>,
    Json(todo): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<TodoRec>), StatusCode> {
    if todo.title.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    match sqlx::query_as!(
        TodoRec,
        r#"
        INSERT INTO todos (title, todo_text, priority, due_date, user_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        todo.title,
        todo.todo_text,
        todo.priority.unwrap_or(1),
        todo.due_date,
        1 // Hardcoded user_id for now
    )
    .fetch_one(&pool)
    .await
    {
        Ok(todo) => {
            info!("Created todo {}", todo.todo_id);
            Ok((StatusCode::CREATED, Json(todo)))
        }
        Err(e) => {
            error!("Database error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Update an existing todo
#[utoipa::path(
    put,
    path = "/todos/{id}",
    request_body = UpdateTodoRequest,
    responses(
        (status = 200, description = "Todo updated successfully", body = TodoRec),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "Todo ID")
    ),
    tag = "todos"
)]
#[instrument(skip(pool))]
pub async fn update_todo(
    State(pool): State<PgPool>,
    Path(todo_id): Path<i32>,
    Json(todo): Json<UpdateTodoRequest>,
) -> Result<Json<TodoRec>, StatusCode> {
    match sqlx::query_as!(
        TodoRec,
        r#"
        UPDATE todos
        SET title = COALESCE($1, title),
            todo_text = COALESCE($2, todo_text),
            completed = COALESCE($3, completed),
            priority = COALESCE($4, priority),
            due_date = COALESCE($5, due_date),
            updated_at = CURRENT_TIMESTAMP
        WHERE todo_id = $6 AND user_id = $7
        RETURNING *
        "#,
        todo.title,
        todo.todo_text,
        todo.completed,
        todo.priority,
        todo.due_date,
        todo_id,
        1 // Hardcoded user_id for now
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(todo)) => {
            info!("Updated todo {}", todo_id);
            Ok(Json(todo))
        }
        Ok(None) => {
            info!("Todo {} not found for update", todo_id);
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("Database error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Delete a todo
#[utoipa::path(
    delete,
    path = "/todos/{id}",
    responses(
        (status = 204, description = "Todo deleted successfully"),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "Todo ID")
    ),
    tag = "todos"
)]
#[instrument(skip(pool))]
pub async fn delete_todo(
    State(pool): State<PgPool>,
    Path(todo_id): Path<i32>,
) -> StatusCode {
    match sqlx::query!(
        "DELETE FROM todos WHERE todo_id = $1 AND user_id = $2",
        todo_id,
        1 // Hardcoded user_id for now
    )
    .execute(&pool)
    .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                info!("Todo {} not found for deletion", todo_id);
                StatusCode::NOT_FOUND
            } else {
                info!("Deleted todo {}", todo_id);
                StatusCode::NO_CONTENT
            }
        }
        Err(e) => {
            error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

