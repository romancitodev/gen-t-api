use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
pub struct Job {
    name: String,
    done: bool,
}

#[derive(Serialize)]
pub struct Jobs {
    jobs: Vec<Job>,
}

#[get("/user/<id>/jobs")]
pub fn get_jobs(id: u8) -> Json<Jobs> {
    if id == 1 {
        Json(Jobs { jobs: vec![] })
    } else if id == 2 {
        Json(Jobs {
            jobs: vec![
                Job {
                    name: "Clean the kitchen".into(),
                    done: false,
                },
                Job {
                    name: "Go to shop".into(),
                    done: true,
                },
            ],
        })
    } else {
        Json(Jobs {
            jobs: vec![Job {
                name: "Go to sleep".into(),
                done: false,
            }],
        })
    }
}
