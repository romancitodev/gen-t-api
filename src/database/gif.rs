use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

/// HTTP Response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub _id: String,
    /// Reference the _id field in Mongo Document
    pub id: u32,
    pub url: String,
    pub name: String,
    pub category: String,
}

/// Document Model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelDocument {
    pub _id: ObjectId,
    pub id: u32,
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
            _id: _id.to_string(),
            id,
            url,
            name,
            category,
        })
    }
}
