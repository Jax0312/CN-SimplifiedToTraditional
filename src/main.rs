#[macro_use] extern crate rocket;
use std::{collections::HashMap};
use rocket::serde::json::Json;
use serde::Serialize;
mod map_simple_to_traditional;

#[derive(Serialize)]

pub struct Payload {
    pub data: HashMap<String, String>,
}

#[get("/<data>")]
fn simplified_to_traditional(data: &str) -> Json<Payload>{
    Json(Payload{data: map_simple_to_traditional::generate_map(data.trim().split("").collect::<Vec<&str>>())})
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![simplified_to_traditional])
}

