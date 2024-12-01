use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TodoRec {
    #[schema(example = 1)]
    pub todo_id: i32,
    #[schema(example = "Buy groceries")]
    pub title: String,
    #[schema(example = "Milk, bread, eggs")]
    pub todo_text: Option<String>,
    #[schema(example = false)]
    pub completed: Option<bool>,
    #[schema(example = 1)]
    pub priority: Option<i32>,
    pub due_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    #[schema(example = 1)]
    pub user_id: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTodoRequest {
    #[schema(example = "Buy groceries")]
    pub title: String,
    #[schema(example = "Milk, bread, eggs")]
    pub todo_text: Option<String>,
    #[schema(example = 1)]
    pub priority: Option<i32>,
    pub due_date: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTodoRequest {
    #[schema(example = "Buy groceries")]
    pub title: Option<String>,
    #[schema(example = "Milk, bread, eggs")]
    pub todo_text: Option<String>,
    #[schema(example = false)]
    pub completed: Option<bool>,
    #[schema(example = 1)]
    pub priority: Option<i32>,
    pub due_date: Option<NaiveDateTime>,
}
