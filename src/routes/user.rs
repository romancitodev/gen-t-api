use bson::doc;
use mongodb::Database;
use rocket::{serde::json::Json, State};

use crate::database::gif::{Model, ModelDocument};

#[get("/gif/<id>")]
pub async fn get_gif_id(db: &State<Database>, id: u32) -> Result<Json<Model>, String> {
    let db = db.collection::<ModelDocument>("gifs");
    println!("{}", id);
    let result = db
        .find_one(
            doc! {
                "id": id
            },
            None,
        )
        .await;

    match result {
        Ok(document) => match document {
            Some(doc) => Ok(doc.into()),
            None => Err("404 Gif not found".to_string()),
        },
        Err(_) => Err("400 Bad request".to_string()),
    }
}

// TODO : finish the post method
// #[post("/gif", data = "<input>")]
// pub async fn post_gif(db: &State<Database>, input: Json<Model>) {
//     let db = db.collection::<ModelDocument>("gifs");
//     db.insert_one(doc! {}, None);
// }
