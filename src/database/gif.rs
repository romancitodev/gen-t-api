use bson::doc;
use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

/// HTTP Response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub id: Option<i32>,
    pub url: String,
    pub name: String,
    pub category: String,
}

/// Document Model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelDocument {
    pub _id: ObjectId,
    pub id: i32,
    pub url: String,
    pub name: String,
    pub category: String,
}

impl From<ModelDocument> for Json<Model> {
    fn from(value: ModelDocument) -> Self {
        let ModelDocument {
            _id,
            id,
            url,
            name,
            category,
        } = value;
        Json(Model {
            id: Some(id),
            url,
            name,
            category,
        })
    }
}

impl From<Json<Model>> for ModelDocument {
    fn from(value: Json<Model>) -> Self {
        Self {
            _id: ObjectId::new(),
            id: value.id.unwrap_or(0),
            url: value.url.to_owned(),
            name: value.name.to_owned(),
            category: value.category.to_owned(),
        }
    }
}
