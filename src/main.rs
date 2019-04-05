#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::content::Json;
use std::collections;
use std::hash::Hash;

enum Tag {
    OOP
}

impl Tag {
    fn to_str(&self) -> &'static str {
        match self {
            Tag::OOP => return "oop",
        }
    }
}

enum Level {
    JUNIOR,
    MEDIUM,
    SENIOR,
}

impl Level {
    fn to_str(&self) -> &'static str {
        match self {
            Level::JUNIOR => return "Junior",
            Level::MEDIUM => return "Medium",
            Level::SENIOR => return "Senior"
        }
    }
}

struct Question {
    id: Hash,
    question: String,
    answer: String,
    level: Level,
    tags: Vec<Tag>,
}

struct Filter {
    level: Level,
    tags: Vec<Tag>,
}

trait QuestionsRepo {
    fn create(question: Question) -> Result<(), E>;
    fn get(filter: Filter) -> Option<Vec<Question>>;
    fn update(question: Question) -> Result<(), E>;
    fn delete(id: Box<Hash>) -> Result<(), E>;
}

impl QuestionRepo {
    fn create(question: Question) -> Result<(), E> {}

    fn get(filter: Filter) -> Option<Vec<Question>> {}

    fn update(question: Question) -> Result<(), E> {}

    fn delete(id: Box<Hash>) -> Result<(), E> {}
}

#[get("/")]
fn index() -> Json<&'static str> {
    Json("{'routes' : ['/get/<topic>/<level>/<tag>']}")
}

#[get("/get")]
fn get() -> Json<&'static str> {
    Json("{'routes' : ['/get/<topic>/<level>/<tag>']}")
}

fn main() {
    let routes = routes![index,
        get,
    ];
    rocket::ignite().mount("/", routes).launch();
}