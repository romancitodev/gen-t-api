use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use rocket::fairing::AdHoc;

pub mod gif;

pub fn init() -> AdHoc {
    AdHoc::on_ignite("Connecting to MongoDB", |rocket| async {
        match connect().await {
            Ok(database) => rocket.manage(database),
            Err(error) => {
                panic!("Cannot connect to instance:: {:?}", error)
            }
        }
    })
}
async fn connect() -> mongodb::error::Result<Database> {
    let [user, password] = [
        std::env::var("MONGO_USR").expect("mongo user not founded"),
        std::env::var("MONGO_PWD").expect("mongo password not founded"),
    ];
    let uri = format!(
        "mongodb+srv://{}:{}@gen-t-api.fvjfjtt.mongodb.net/?retryWrites=true&w=majority&authSource=admin",
        user, password
    );
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("giphy");
    println!("MongoDB Connected!");
    Ok(database)
}
