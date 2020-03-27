mod models;

use std::io;
use actix_web::{HttpServer, web, Responder, App};
use crate::models::Status;

async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status{
            status: "OK".to_string()
        })
}

#[actix_rt::main]
async fn main() -> io::Result<()>{
    println!("Starting server at http://0.0.0.0:8080");
    HttpServer::new(||{
        App::new()
            .route("/", web::get()
                .to(status))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
