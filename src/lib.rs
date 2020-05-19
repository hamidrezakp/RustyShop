#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate chrono;

pub mod database;
mod models;
mod schema;

#[database("sqlite_main")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn run_rocket() {
    /*rocket::ignite()
    .mount("/", routes![index])
    .attach(DbConn::fairing())
    .launch(); */
}
