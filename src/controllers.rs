use crate::models::Question;
use rocket_contrib::json::Json;
use crate::libs::QuestionRepo;
use rocket::State;
use crate::libs::session::Session;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashSet;

#[get("/")]
pub fn index() -> Json<&'static str> {
    Json("{'routes' : ['/get/<topic>/<level>/<tag>']}")
}

pub fn randomize_questions(state: State<Session>, amount: u16) -> Option<Vec<Question>> {
    let mut rng = rand::thread_rng();
    let gen_num = Uniform::from(1..amount);
    let mut my_set = HashSet::new();
    for _ in 0..amount {
        while !my_set.insert(gen_num.sample(&mut rng)) {}
    }
    for it in my_set {

    }
    match QuestionRepo::read(state) {
        None => None,
        Some(questions) => if amount <= questions.len() { questions. } else { None },
    }
}

#[get("/random?<amount>&<level>&<topic>")]
pub fn random(state: State<Session>, amount: u16, level: String, topic: Option<String>) -> Json<String> {
    match randomize_questions(state, amount) {
        None => Json(format!("Empty database.")),
        Some(questions) => Json(serde_json::to_string(&questions).expect("Error")),
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
