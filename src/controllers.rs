use crate::models::Question;
use rocket_contrib::json::Json;

#[get("/")]
pub fn index() -> Json<&'static str> {
    Json("{'routes' : ['/get/<topic>/<level>/<tag>']}")
}

pub fn randomize_questions() -> Vec<Question> {
    vec![]
}

#[get("/random?<amount>&<level>&<topic>")]
pub fn random(amount: u16, level: String, topic: Option<String>) -> Json<String> {
    //Json(serde_json::to_string(&randomize_questions()).expect("Error while randomize."))
    unimplemented!()
}

pub mod question {
    use crate::libs::session::Session;
    use crate::models::Question;
    use crate::QuestionRepo;
    use core::borrow::Borrow;
    use rocket::State;
    use rocket_contrib::json::Json;
    use std::sync::Arc;

    #[get("/question")]
    pub fn read(state: State<Session>) -> Json<String> {
        if !state
            .session_dic
            .read()
            .expect("No state")
            .contains_key("obj")
        {
            match QuestionRepo::read() {
                None => None,
                Some(q) => state
                    .session_dic
                    .write()
                    .expect("No state")
                    .insert("obj", q),
            };
        }
        match state
            .session_dic
            .read()
            .expect("No state")
            .get("obj")
            .and_then(|q| Some(q.to_vec()))
        {
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
