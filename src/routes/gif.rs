use bson::doc;
use mongodb::Database;
use rocket::{serde::json::Json, State};

use crate::{
    database::{
        gif::{Model, ModelDocument},
        incremental::AutoIncrement,
    },
    response::{HttpResult, Response, Status},
};

#[get("/gif/<id>")]
pub async fn get_gif_id(db: &State<Database>, id: u32) -> HttpResult<ModelDocument> {
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
            Some(doc) => Response::build(Status::Accepted, doc),
            None => Response::build_err(Status::NotFound, "Gif not founded".into()),
        },
        Err(err) => Response::build_err(Status::BadRequest, format!("Bad request: {}", err)),
    }
}

#[post("/gif", data = "<input>", format = "json")]
pub async fn post_gif(db: &State<Database>, input: Json<Model>) -> HttpResult<ModelDocument> {
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
        Err(response) => return Response::build_err(Status::BadRequest, response),
    };

    match gif_doc.insert_one(doc.clone(), None).await {
        Ok(_) => Response::build(Status::Accepted, doc),
        Err(err) => Response::build_err(Status::BadRequest, err.to_string()),
    }
}
