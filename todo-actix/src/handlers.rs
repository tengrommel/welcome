use actix_web::{Responder, web, HttpResponse};
use crate::models::{Status, CreateTodoList, ResultResponse, AppState};
use deadpool_postgres::{Pool, Client};
use crate::db;
use crate::errors::AppError;
use slog::{o, crit, Logger};

pub async fn get_client(pool: Pool, log: Logger) -> Result<Client, AppError> {
    pool.get().await
        .map_err(|err|{
            let sublog = log.new(o!("cause" => err.to_string()));
            crit!(sublog, "Error creating client");
            AppError::db_error(err)
        })
}

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status{
            status: "Up".to_string()
        })
}

pub async fn get_todos(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "get_todos"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;
    let result = db::get_todos(&client).await;
    result.map(|todos|HttpResponse::Ok().json(todos))
}

pub async fn get_items(state: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "get_todos"));
    let client: Client =  get_client(state.pool.clone(), log.clone()).await?;
    let result = db::get_items(&client, path.0).await;
    result.map(|items|HttpResponse::Ok().json(items))
}

pub async fn create_todo(state: web::Data<AppState>, json: web::Json<CreateTodoList>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "get_todos"));
    let client: Client =  get_client(state.pool.clone(), log.clone()).await?;
    let result = db::create_todo(&client, json.title.clone()).await;
    result.map(|todo| HttpResponse::Ok().json(todo))
}

pub async fn check_item(state: web::Data<AppState>, path: web::Path<(i32, i32)>) -> Result <impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "get_todos"));
    let client: Client =  get_client(state.pool.clone(), log.clone()).await?;
    let result = db::check_todo(&client, path.0, path.1).await;
    result.map(|updated| HttpResponse::Ok().json(ResultResponse{success: updated}))
}