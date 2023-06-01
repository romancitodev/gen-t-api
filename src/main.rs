#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
mod database;
mod routes;

use routes::*;

#[launch]
async fn rocket() -> _ {
    match dotenv() {
        Ok(_) => {}
        Err(_) => println!("File .env not founded, the program can crash"),
    }
    rocket::build()
        .attach(database::init())
        .attach(CORS)
        .mount("/", routes![root, get_gif_id])
}

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
