#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
mod routes;

use routes::*;

#[launch]
async fn rocket() -> _ {
    dotenv().expect("`.env` File not found...");
    let [user, password] = [
        std::env::var("MONGO_USR").expect("mongo user not founded"),
        std::env::var("MONGO_PWD").expect("mongo password not founded"),
    ];
    let uri = format!(
        "mongodb+srv://{}:{}@gen-t-api.fvjfjtt.mongodb.net/?retryWrites=true&w=majority",
        user, password
    );
    let mut mongo_options = ClientOptions::parse(uri).await.expect("Error parsing uri");
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    mongo_options.server_api = Some(server_api);
    let client = Client::with_options(mongo_options).expect("Error setting mongo client");
    let ping = client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await
        .expect("Error pinging to MongoDB...");

    println!("{:?}", ping);

    rocket::build().mount("/", routes![root, user_id, get_jobs])
}
