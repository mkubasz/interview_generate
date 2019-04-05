#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content::Json;
use std::collections;
use std::hash::Hash;

enum Tag {
    OOP
}

enum Level {
    JUNIOR,
    MEDIUM,
    SENIOR
}

struct Question {
    id: Hash,
    question: String,
    answer: String,
    level: Level,
    tags: Vec<Tag>
}

struct Filter {
    level: Level,
    tags: Vec<Tag>
}

trait QuestionsRepo {
    fn create(question: Question);
    fn get(filter: Filter);
    fn update(question: Question);
    fn delete(id: Box<Hash>);
}

impl QuestionRepo {
    // TODO
}

#[get("/")]
fn index() -> Json<&'static str> {
    Json("{'routes' : ['/get/<topic>/<level>/<tag>']}")
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}