#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::collections;
use std::result;
use std::hash::Hash;
use rocket::response::content::Json;


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
    id: i32,
    question: String,
    answer: String,
    level: Level,
    tags: Vec<Tag>,
}

struct Filter {
    number: i32,
    level: Level,
    tags: Vec<Tag>,
}


trait QuestionRepo {
    fn create(question: Question) -> Result<(), ()> {
        Ok(())
    }

    fn get(filter: Filter) -> Option<Vec<Question>> {
        None
    }

    fn update(question: Question) -> Result<(), ()> {
        Ok(())
    }

    fn delete(id: i32) -> Result<(), ()> {
        Ok(())
    }
}

#[get("/")]
fn index() -> Json<&'static str> {
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