use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub todo_id: i32,
    pub title: String,
    pub todo_text: Option<String>,
    pub completed: Option<bool>,
    pub priority: Option<i32>,
    pub due_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub user_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub todo_text: Option<String>,
    pub priority: Option<i32>,
    pub due_date: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub todo_text: Option<String>,
    pub completed: Option<bool>,
    pub priority: Option<i32>,
    pub due_date: Option<NaiveDateTime>,
} 