use bson::doc;
use futures::{StreamExt, TryStreamExt};
use mongodb::{options::FindOptions, Database};
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

#[get("/gifs/<id>")]
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
            None => ResponseBuilder::build_err(Status::NotFound, "Gif not found"),
        },
        Err(err) => ResponseBuilder::build_err(Status::BadRequest, format!("Bad request: {}", err)),
    }
}

#[post("/gifs", data = "<input>", format = "json")]
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

#[post("/gifs", rank = 2)]
pub async fn post_gif_id_unauthorized() -> Redirect {
    Redirect::to(uri!("/api/v1/auth"))
}

#[derive(Debug, FromForm)]
pub struct QueryOptions {
    page: Option<u64>,
    limit: Option<i64>,
}

#[get("/gifs?<options..>")]
pub async fn get_all_gifs(
    db: &State<Database>,
    _auth: Auth,
    options: QueryOptions,
) -> HttpResult<Vec<ModelDocument>> {
    let gif_doc = db.collection::<ModelDocument>("gifs");
    let max_gifs = options.limit.or(Some(25)).min(Some(50)); // establecemos que el maximo de gifs van a ser 50
    let find_options = FindOptions::builder()
        .limit(max_gifs)
        .skip(options.page.unwrap_or(0) * max_gifs.unwrap() as u64);
    let find_options = find_options.build();
    let cursor = match gif_doc.find(None, find_options).await {
        Ok(c) => c,
        Err(err) => {
            return ResponseBuilder::build_err(
                Status::Custom(500),
                format!("Error listing document: {err}"),
            )
        }
    };
    let gifs = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
    ResponseBuilder::build(Status::Accepted, gifs)
}
