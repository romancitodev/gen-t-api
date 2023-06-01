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
        .mount("/", routes![root, get_gif_id])
}
