#![feature(proc_macro_hygiene, decl_macro)]

mod controllers;
mod lib;
mod models;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate strum_macros;
extern crate strum;

use crate::models::Question;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

struct JsonProvider {
    file_name: &'static str,
}

impl JsonProvider {
    fn parse_file(&self) -> Option<Vec<Question>> {
        match File::open(&self.file_name) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Error");
                let parsed: models::File = serde_json::from_str(&contents).expect("Error");
                return Some(parsed.questions);
            }
            Err(e) => None,
        }
    }

    fn add_item(question: Question) -> Result<(), ()> {
        Ok(())
    }

    fn update_item() -> Result<(), ()> {
        Ok(())
    }

    fn delete_item() -> Result<(), ()> {
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
        let j = JsonProvider {
            file_name: "src/data/interview.json",
        };
        j.parse_file()
    }

    fn update(question: Question) -> Result<(), ()> {
        JsonProvider::update_item()
    }

    fn delete(id: Uuid) -> Result<(), ()> {
        JsonProvider::delete_item()
    }
}

fn main() {
    let routes = routes![
        controllers::index,
        controllers::random,
        controllers::question::read,
        controllers::question::create,
        controllers::question::update,
        controllers::question::delete
    ];
    rocket::ignite().mount("/", routes).launch();
}
