#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate chrono;

mod database;
mod models;
mod schema;
mod website;

pub fn run_website() {
    website::run_rocket();
}
