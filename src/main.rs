#[macro_use]
use std::{collections::HashMap};
use serde::Serialize;
use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, http};
mod map_simple_to_traditional;

#[derive(Serialize)]
pub struct Payload {
    pub data: HashMap<String, String>,
}

#[get("/{data}")]
async fn get_message(data: web::Path<String>) -> impl Responder {
    // // Serialize the message to JSON and return it as the response
    let payload = Payload{data: map_simple_to_traditional::generate_map(data.trim().split("").collect::<Vec<&str>>())};
    HttpResponse::Ok().json(payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {

        let cors = Cors::default().allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new().wrap(cors)
            .service(get_message)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}




