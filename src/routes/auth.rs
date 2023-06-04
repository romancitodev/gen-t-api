use bson::{oid::ObjectId, DateTime};

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::Database;
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::{
    database::token::ModelDocument,
    response::{HttpResult, ResponseBuilder, Status},
};

#[post("/auth")]
pub async fn post_auth(db: &State<Database>) -> HttpResult<ModelDocument> {
    let binding = std::env::var("JWT_KEY").unwrap();
    let secret_key = binding.as_bytes();
    let token = ModelDocument {
        id: ObjectId::new(),
        token: create_token(secret_key, "User", 120).unwrap(),
        created_at: DateTime::now(),
    };
    let tokens = db.collection("tokens");
    tokens.insert_one(token.clone(), None).await.unwrap();
    ResponseBuilder::build(Status::Accepted, token)
}

#[get("/auth")]
pub fn get_auth() -> HttpResult<()> {
    ResponseBuilder::build_err(Status::Unauthorized, "Missing or invalid Bearer token")
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn create_token(
    secret_key: &[u8],
    subject: &str,
    expiration_minutes: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now() + Duration::minutes(expiration_minutes);

    let claims = Claims {
        sub: subject.to_owned(),
        exp: expiration.timestamp() as usize,
    };

    let header = Header::default();
    encode(&header, &claims, &EncodingKey::from_secret(secret_key))
}
