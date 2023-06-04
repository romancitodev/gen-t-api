use bson::doc;
use mongodb::Database;
use rocket::{response::Redirect, serde::json::Json, State};

use crate::{
    auth::Auth,
    database::{
        gif::{Model, ModelDocument},
        incremental::AutoIncrement,
    },
    response::{HttpResult, ResponseBuilder, Status},
};

#[catch(401)]
pub async fn get_gif_id_unauthorized() -> Redirect {
    Redirect::to(uri!("/api/v1/auth"))
}

#[get("/gif/<id>")]
pub async fn get_gif_id(db: &State<Database>, _auth: Auth, id: u32) -> HttpResult<ModelDocument> {
    let db = db.collection::<ModelDocument>("gifs");
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
            Some(doc) => ResponseBuilder::build(Status::Accepted, doc),
            None => ResponseBuilder::build_err(Status::NotFound, "Gif not found".into()),
        },
        Err(err) => ResponseBuilder::build_err(Status::BadRequest, format!("Bad request: {}", err)),
    }
}

// TODO : implement Bearer tokens
#[post("/gif", data = "<input>", format = "json")]
pub async fn post_gif(
    db: &State<Database>,
    input: Json<Model>,
    _auth: Auth,
) -> HttpResult<ModelDocument> {
    let gif_doc = db.collection::<ModelDocument>("gifs");
    let inc_doc = db.collection::<AutoIncrement>("counter");
    let mut auto = AutoIncrement {
        collection: "gifs".into(),
        seq: 0,
    };

    let mut doc: ModelDocument = input.into();
    match auto.get_next_id(inc_doc).await {
        Ok(id) => {
            doc.id = id;
        }
        Err(response) => return ResponseBuilder::build_err(Status::BadRequest, response),
    };

    match gif_doc.insert_one(doc.clone(), None).await {
        Ok(_) => ResponseBuilder::build(Status::Accepted, doc),
        Err(err) => ResponseBuilder::build_err(Status::BadRequest, err.to_string()),
    }
}

#[post("/gif", rank = 2)]
pub async fn post_gif_id_unauthorized() -> Redirect {
    Redirect::to(uri!("/api/v1/auth"))
}
