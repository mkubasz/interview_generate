use crate::libs;
use crate::libs::session::Session;
use crate::libs::QuestionRepo;
use crate::models::Question;
use rocket::State;
use rocket_contrib::json::Json;

#[get("/")]
pub fn index() -> Json<&'static str> {
    Json("{'routes' : ['/get/<topic>/<level>/<tag>']}")
}

#[get("/random?<amount>&<level>&<topic>")]
pub fn random(
    state: State<Session>,
    amount: u64,
    level: String,
    topic: Option<String>,
) -> Json<String> {
    match libs::randomize_questions(state, amount) {
        Err(err) => Json(err),
        Ok(questions) => Json(serde_json::to_string(&questions).expect("Error")),
    }
}

pub mod question {
    use crate::libs::session::Session;
    use crate::libs::QuestionRepo;
    use crate::models::Question;
    use core::borrow::Borrow;
    use rocket::State;
    use rocket_contrib::json::Json;
    use std::sync::Arc;

    #[get("/question")]
    pub fn read(state: State<Session>) -> Json<String> {
        match QuestionRepo::read(state) {
            None => Json(format!("Empty database.")),
            Some(questions) => Json(serde_json::to_string(&questions).expect("Error")),
        }
    }

    #[post("/question")]
    pub fn create() -> Json<String> {
        Json(format!("Create"))
    }

    #[put("/question")]
    pub fn update() {}

    #[delete("/question/<id>")]
    pub fn delete(id: String) {
        match uuid::Uuid::parse_str(id.as_str()) {
            Ok(id) => {
                // TODO
                QuestionRepo::delete(id).unwrap()
            }
            Err(_) => {}
        }
    }
}
