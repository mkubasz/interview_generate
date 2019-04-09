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
    use crate::QuestionRepo;
    use rocket_contrib::json::Json;

    #[get("/question")]
    pub fn read() -> Json<String> {
        let questions = QuestionRepo::read();
        match questions {
            None => Json(format!("Empty database.")),
            //Some(q) => //serde_json::to_string(&q),
            Some(q) => unimplemented!(),
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
