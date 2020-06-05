use crate::database;
use crate::models;
use crate::rocket;

use rocket::request::Form;
use rocket_contrib::{
    json::Json,
    serve::StaticFiles,
    templates::{tera::Context, Template},
};

#[database("sqlite_main")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn index(conn: DbConn) -> Template {
    let mut context = Context::new();
    context.insert("title", "Rusty Shop");
    let products_to_show = database::get_all_products(&conn, 20);
    context.insert("products", &products_to_show);
    context.insert("page", "index");
    Template::render("index", &context)
}

#[get("/dashboard")]
fn dashboard(conn: DbConn) -> Template {
    let mut context = Context::new();
    context.insert("title", "Dashboard");
    context.insert("page", "dashboard");
    Template::render("dashboard", &context)
}

#[get("/checkout")]
fn checkout(conn: DbConn) -> Template {
    let mut context = Context::new();
    context.insert("title", "Checkout");
    context.insert("page", "checkout");
    Template::render("checkout", &context)
}

#[post("/signup", data = "<user>")]
fn signup(conn: DbConn, user: Form<models::FormUser>) {
    database::insert_user(&conn, user.into_inner());
}

#[post("/checkout", data = "<checkout_form>")]
fn order(conn: DbConn, mut checkout_form: Json<models::CheckoutForm>) {
    checkout_form.products.sort_by(|a, b| a.0.cmp(&b.0));

    let payment_amount = database::get_products_price(&conn, &checkout_form.products);

    let order_id =
        database::insert_and_get_order(&conn, &checkout_form.address, &checkout_form.phonenumber);

    let payment = database::insert_and_get_payment(
        &conn,
        payment_amount,
        order_id.unwrap(),
        checkout_form.user_id,
    );

    database::insert_products_with_order(&conn, &checkout_form.products, order_id.unwrap());
}

pub fn run_rocket() {
    rocket::ignite()
        .mount("/", routes![index, dashboard, checkout, signup, order])
        .mount(
            "/assets",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/files")),
        )
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .launch();
}
