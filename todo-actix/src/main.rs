mod models;
mod config;
mod handlers;
mod db;
mod errors;

use std::io;
use actix_web::{HttpServer, web, App};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::{status, get_todos, get_items, create_todo, check_item};

#[actix_rt::main]
async fn main() -> io::Result<()>{
    dotenv().ok();
    let configs = crate::config::Config::from_env().unwrap();
    let pool = configs.pg.create_pool(NoTls).unwrap();
    println!("Starting server at http://{}:{}", configs.server.host, configs.server.port);
    HttpServer::new(move ||{
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/todos{_:/?}", web::post().to(create_todo))
            .route("/todos/{list_id}/items", web::get().to(get_items))
            .route("/todos/{list_id}/items/{item_id}{_:/?}", web::put().to(check_item))
    })
        .bind(format!("{}:{}", configs.server.host, configs.server.port))?
        .run()
        .await
}