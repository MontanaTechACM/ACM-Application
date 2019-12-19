#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket_failure;
extern crate chrono;
extern crate dotenv;
extern crate crypto;
extern crate jwt;
extern crate rustc_serialize;
extern crate regex;

mod endpoints;
mod models;
mod schema;
mod db;
mod auth;

pub fn init_rocket() -> rocket::Rocket {
    rocket::ignite().manage(db::connect())
}

fn main() {
    let mut rocket = init_rocket();
    rocket = endpoints::login::mount(rocket);
    rocket = endpoints::event::mount(rocket);
    rocket.launch();
}
