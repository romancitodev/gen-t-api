use bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub token: String,
    pub created_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelDocument {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub token: String,
    pub created_at: DateTime,
}
