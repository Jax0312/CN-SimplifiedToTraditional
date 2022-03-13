#![feature(in_band_lifetimes)]
#[macro_use] extern crate rocket;
use std::{collections::HashMap};
use rocket::serde::json::Json;
use rocket::http::Header;
use rocket::{Request, Response};
use serde::Serialize;
use rocket::fairing::{Fairing, Info, Kind};
mod map_simple_to_traditional;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

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
    rocket::build().mount("/", routes![simplified_to_traditional]).attach(CORS)
}

