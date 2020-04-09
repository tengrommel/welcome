use actix_web::{Responder, web, HttpResponse};
use crate::models::{Status, CreateTodoList, ResultResponse};
use deadpool_postgres::{Pool, Client};
use crate::db;
use std::io::ErrorKind::Other;
use crate::errors::{AppError, AppErrorType};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status{
            status: "Up".to_string()
        })
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
    let client: Client =
        db_pool.get().await
            .map_err(
                |err| AppError {message: None, cause: Some(err.to_string()),
                error_type: AppErrorType::DbError})?;
    let result = db::get_todos(&client).await;
    result.map(|todos|HttpResponse::Ok().json(todos))
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to the database");
    let result = db::get_items(&client, path.0).await;
    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to the database");
    let result = db::create_todo(&client, json.title.clone()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn check_item(db_pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to the database");
    let result = db::check_todo(&client, path.0, path.1).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(ResultResponse{success: true}),
        Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(ResultResponse{success: false}),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}