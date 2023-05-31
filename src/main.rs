#[macro_use]
extern crate rocket;

mod routes;

use routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![root, user_id, get_jobs])
}
