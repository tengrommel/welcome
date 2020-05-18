use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use slog::Logger;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub log: Logger,
}

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo_list")]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo_item")]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32,
}

#[derive(Deserialize)]
pub struct CreateTodoList {
    pub title: String,
}

#[derive(Serialize)]
pub struct ResultResponse {
    pub success: bool,
}
