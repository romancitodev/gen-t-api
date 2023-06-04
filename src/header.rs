// use mongodb::Database;
// use rocket::request::Outcome;
// use rocket::request::{FromRequest, Request};
// use rocket::response::status;
// use rocket::serde::json::Json;
// use serde::{Deserialize, Serialize};

// use crate::response::Response;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Token {
//     /// expiration
//     pub expires_at: i64,
//     pub token: String,
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for Token {
//     type Error = status::Custom<Json<Response<String>>>;

//     async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//         // let mongo = request.guard::<Database>().await;
//         todo!()
//     }
// }
