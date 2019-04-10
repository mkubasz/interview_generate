#![feature(proc_macro_hygiene, decl_macro)]

pub mod controllers;
pub mod libs;
pub mod models;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate strum_macros;
extern crate strum;

use crate::libs::session::Session;

fn main() {
    let routes = routes![
        controllers::index,
        controllers::random,
        controllers::question::read,
        controllers::question::create,
        controllers::question::update,
        controllers::question::delete
    ];

    let session = Session::new();

    rocket::ignite().mount("/", routes).manage(session).launch();
}
