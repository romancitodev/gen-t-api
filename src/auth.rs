use bson::doc;
use mongodb::Database;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request, State,
};

use crate::database::token::ModelDocument;

#[derive(Debug, PartialEq)]
struct AuthToken {
    token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");

        match auth_header {
            Some(header_value) => {
                if header_value.starts_with("Bearer ") {
                    let token = header_value.trim_start_matches("Bearer ").into();
                    Outcome::Success(AuthToken { token })
                } else {
                    Outcome::Failure((Status::Unauthorized, ()))
                }
            }
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

#[derive(Debug)]
pub struct Auth;
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(database) = request.guard::<&State<Database>>().await.succeeded() {
            let tokens = database.collection::<ModelDocument>("tokens");
            let token_header = match AuthToken::from_request(request).await {
                Outcome::Failure((status, _)) => return Outcome::Failure((status, ())),
                Outcome::Forward(_) => return Outcome::Forward(()),
                Outcome::Success(t) => t,
            };

            match tokens
                .find_one(
                    doc! {
                        "token": token_header.token
                    },
                    None,
                )
                .await
            {
                Ok(None) => Outcome::Failure((Status::Unauthorized, ())),
                Ok(Some(_)) => Outcome::Success(Auth),
                Err(_) => Outcome::Failure((Status::Unauthorized, ())),
            }
        } else {
            Outcome::Forward(())
        }
    }
}
