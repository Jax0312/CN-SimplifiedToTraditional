#[macro_use]
extern crate rocket;
use std::{collections::HashMap};
use rocket::serde::json::Json;
use rocket::http::Method;
use serde::Serialize;
use std::error::Error;
use rocket_cors::{AllowedOrigins, CorsOptions};
mod map_simple_to_traditional;

#[derive(Serialize)]
pub struct Payload {
    pub data: HashMap<String, String>,
}

#[get("/<data>")]
fn simplified_to_traditional(data: &str) -> Json<Payload>{
    Json(Payload{data: map_simple_to_traditional::generate_map(data.trim().split("").collect::<Vec<&str>>())})
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()?;

    rocket::build().mount("/", routes![simplified_to_traditional]).attach(cors).launch().await?;

    Ok(())

}

