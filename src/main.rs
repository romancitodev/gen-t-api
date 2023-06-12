#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
mod auth;
mod database;
mod response;
mod routes;

use routes::*;

#[launch]
async fn rocket() -> _ {
    if let Err(e) = dotenv() {
        println!("File .env not found, the program can crash, reason {e}")
    };
    rocket::build()
        .attach(database::init())
        .attach(CORS)
        .mount(
            "/api/v1/",
            routes![
                get_gif_id,
                post_gif_id_unauthorized,
                post_gif,
                get_auth,
                post_auth,
                get_all_gifs,
                handle_options
            ],
        )
        .register("/api/v1/gifs", catchers![get_gif_id_unauthorized])
    // .ignite()
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
            "GET, POST, PUT, DELETE, PATCH",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Authorization, Content-Type",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
