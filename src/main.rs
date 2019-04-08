#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate strum_macros;
extern crate uuid;
extern crate strum;

use std::fs;
use std::result;
use std::hash::Hash;
use std::collections;
use uuid::Uuid;
use rocket::response::content::Json;
use strum::EnumMessage;

#[derive(Display)]
enum Tag {
    OOP
}

#[derive(Display)]
enum Level {
    JUNIOR,
    MEDIUM,
    SENIOR,
}

struct Question {
    id: Option<Uuid>,
    question: String,
    answer: String,
    level: Level,
    tags: Vec<Tag>,
}

struct Filter {
    number: u16,
    level: Level,
    tags: Vec<Tag>,
}

struct JsonProvider{
    file_name: String
}

impl JsonProvider {
    fn parse_file() -> Option<Vec<Question>> {
        None
    }

    fn add_item(question: Question) -> Result<(),()>{
        Ok(())
    }

    fn update_item() -> Result<(),()>{
        Ok(())
    }

    fn delete_item() -> Result<(),()>{
        Ok(())
    }
}



struct QuestionRepo;

impl QuestionRepo {
    fn create(question: Question) -> Result<(), ()> {
        JsonProvider::add_item(question)
    }

    fn read() -> Option<Vec<Question>> {
        // Implement session
        JsonProvider::parse_file()
    }


    fn update(question: Question) -> Result<(), ()> {
        JsonProvider::update_item()
    }

    fn delete(id: Uuid) -> Result<(),()> {
        JsonProvider::delete_item()
    }
}

#[get("/")]
fn index() -> Json<&'static str> {
    Json("{'routes' : ['/get/<topic>/<level>/<tag>']}")
}

fn randomize_questions() -> Vec<Question> {

}

#[get("/random?<amount>&<level>&<topic>")]
fn random(amount: u16, level: String, topic: Option<String>) -> Json<String> {
    Json(serde_json::to_string(randomize_questions()).expect("Error while randomize."))
}

mod question {
    use rocket_contrib::json::Json;
    use crate::QuestionRepo;

    #[get("/question")]
    pub fn read() -> Json<String> {
        let questions = QuestionRepo::read();
        match questions {
            None => Json(format!("Empty database.")),
            Some(q) => {
                serde_json::to_string(q)
            },
        }
    }

    #[post("/question")]
    pub fn create() -> Json<&'static str> {
    }

    #[put("/question")]
    pub fn update() {

    }

    #[delete("/question/<id>")]
    pub fn delete(id: String) {
        match uuid::Uuid::from_str(id.as_str()) {
            Ok(id) => {
                // TODO
                QuestionRepo::delete(id).unwrap()
            },
            Err(_) => {},
        }
    }
}

fn main() {
    let routes = routes![
        index,
        random,
        question::read,
        question::create,
        question::update,
        question::delete
    ];
    rocket::ignite().mount("/", routes).launch();
}