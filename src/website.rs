use crate::database;
use crate::rocket;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::Template;

#[database("sqlite_main")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn index(conn: DbConn) -> Template {
    let mut context = Context::new();
    context.insert("title", "Rusty Shop");
    let products_to_show = database::get_all_products(&conn, 20);
    context.insert("products", &products_to_show);
    Template::render("index", &context)
}

pub fn run_rocket() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount(
            "/assets",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/files")),
        )
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .launch();
}
