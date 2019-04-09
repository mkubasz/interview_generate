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
use uuid::Uuid;

struct JsonProvider;

impl JsonProvider {
    fn parse_file() -> Option<Vec<Question>> {
        None
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
        JsonProvider::parse_file()
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
