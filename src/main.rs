#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
mod database;
mod routes;

use routes::*;

#[launch]
async fn rocket() -> _ {
    dotenv().expect("`.env` File not found...");
    rocket::build()
        .attach(database::init())
        .mount("/", routes![root, get_gif_id])
}
