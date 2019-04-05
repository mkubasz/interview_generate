#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate uuid;

use std::fs;
use std::result;
use std::hash::Hash;
use std::collections;
use uuid::Uuid;
use rocket::response::content::Json;

enum Tag {
    OOP
}

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

struct QuestionRepo;

impl QuestionRepo {
    fn create(question: Question) -> Result<(), ()> {
        Ok(())
    }

    fn get() -> Option<Vec<&'static str>> {
        Some(vec!["d"])
    }

    fn update(question: Question) -> Result<(), ()> {
        Ok(())
    }

    fn delete(id: Uuid) -> Result<(), ()> {
        Ok(())
    }
}

#[get("/")]
fn index() -> Json<&'static str> {
    Json("{'routes' : ['/get/<topic>/<level>/<tag>']}")
}

#[get("/random?<number>&<level>&<topic>")]
fn random(number: u16, level: String, topic: Option<String>) -> Json<&'static str> {
    let st = format!("{{'routes' : ['/random/<{}>/<{}>/<{:?}>']}}", number, level, topic);
    Json(Box::leak(st.into_boxed_str()))
}

mod question {
    use rocket_contrib::json::Json;

    #[get("/question")]
    pub fn read() -> Json<&'static str> {
        Json("{'routes' : ['/get/<topic>/<level>/<tag>']}")
    }

    #[post("/question")]
    pub fn create() -> Json<&'static str> {
        Json("{'routes' : ['/get/<topic>/<level>/<tag>']}")
    }

    #[put("/question")]
    pub fn update() {}

    #[delete("/question")]
    pub fn delete() {}
}

mod tag {
    use rocket_contrib::json::Json;

    #[get("/tag")]
    pub fn read() -> Json<&'static str> {
        Json("{'routes' : ['/get/<topic>/<level>/<tag>']}")
    }

    #[post("/tag")]
    pub fn create() -> Json<&'static str> {
        Json("{'routes' : ['/get/<topic>/<level>/<tag>']}")
    }

    #[put("/tag")]
    pub fn update() {}

    #[delete("/tag")]
    pub fn delete() {}
}

fn main() {
    let routes = routes![
        index,
        random,
        question::read,
        question::create,
        question::update,
        question::delete,
        tag::read,
        tag::create,
        tag::update,
        tag::delete
    ];
    rocket::ignite().mount("/", routes).launch();
}