use crate::libs;
use crate::libs::session::Session;
use crate::libs::QuestionRepo;
use crate::models::{Question, Tag};
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
    match libs::randomize_questions(state, amount, vec![Tag::JUNIOR]) {
        Err(err) => Json(err),
        Ok(questions) => Json(serde_json::to_string(&questions).expect("Error")),
    }
}

pub mod question {
    use crate::libs::session::Session;
    use crate::libs::QuestionRepo;
    use crate::models::{Question, Tag};
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
    pub fn create(state: State<Session>) -> Json<String> {
        match QuestionRepo::create(
            state,
            Question {
                id: Some(format!("1")),
                question: "fdfd".to_string(),
                answer: "fdfd".to_string(),
                tags: vec![Tag::OOP],
            },
        ) {
            Ok(()) => Json(format!("Create")),
            Err(er) => Json(format!("Error")),
        }
    }

    #[put("/question")]
    pub fn update(state: State<Session>) -> Json<String> {
        QuestionRepo::update(
            state,
            Question {
                id: Some(format!("1")),
                question: "fdfd".to_string(),
                answer: "fdfd".to_string(),
                tags: vec![Tag::OOP],
            },
        );
        Json(format!("Update"))
    }

    #[delete("/question/<id>")]
    pub fn delete(state: State<Session>, id: String) {
        match uuid::Uuid::parse_str(id.as_str()) {
            Ok(id) => QuestionRepo::delete(state, id).unwrap(),
            Err(_) => {}
        }
    }
}
