use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::{CreateTodoRequest, Todo, UpdateTodoRequest};

#[get("/todos")]
pub async fn get_todos(db: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as!(
        Todo,
        "SELECT * FROM todos ORDER BY created_at DESC"
    )
    .fetch_all(db.get_ref()) //ToDO: paginator
    .await
    {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/todos/{id}")]
pub async fn get_todo(db: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    //ToDo: session?  user_id?
    match sqlx::query_as!(
        Todo,
        "SELECT * FROM todos WHERE todo_id = $1",
        id.into_inner()
    )
    .fetch_optional(db.get_ref())
    .await
    {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/todos")]
pub async fn create_todo(
    db: web::Data<PgPool>,
    todo: web::Json<CreateTodoRequest>,
) -> impl Responder {
    //ToDo: session?  user_id?
    match sqlx::query_as!(
        Todo,
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
    .fetch_one(db.get_ref())
    .await
    {
        Ok(todo) => HttpResponse::Created().json(todo),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/todos/{id}")]
pub async fn update_todo(
    db: web::Data<PgPool>,
    id: web::Path<i32>,
    todo: web::Json<UpdateTodoRequest>,
) -> impl Responder {
    match sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET title = COALESCE($1, title),
            todo_text = COALESCE($2, todo_text),
            completed = COALESCE($3, completed),
            priority = COALESCE($4, priority),
            due_date = COALESCE($5, due_date),
            updated_at = CURRENT_TIMESTAMP
        WHERE todo_id = $6
        RETURNING *
        "#,
        todo.title,
        todo.todo_text,
        todo.completed,
        todo.priority,
        todo.due_date,
        id.into_inner()
    )
    .fetch_optional(db.get_ref())
    .await
    {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo(db: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    match sqlx::query!("DELETE FROM todos WHERE todo_id = $1", id.into_inner())
        .execute(db.get_ref())
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
} 