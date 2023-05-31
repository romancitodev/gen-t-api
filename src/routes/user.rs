use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
pub struct User {
    id: u8,
    name: String,
}

#[get("/user/<id>")]
pub fn user_id(id: u8) -> Json<User> {
    let user = User {
        id,
        name: "Bob".into(),
    };
    Json(user)
}
