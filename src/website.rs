use crate::database;
use crate::models;
use crate::rocket;

use rocket::http::Status;
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
    context.insert("orders", &database::get_all_orders(&conn));
    context.insert("products", &database::get_all_products(&conn, 100));
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
fn signup(conn: DbConn, user: Json<models::FormUser>) -> &'static str {
    let result = database::insert_user(&conn, user.into_inner());
    match result {
        Ok(()) => "OK",
        Err(database::DBResult::DuplicateUsername) => "Error: Duplicate Username",
        Err(_) => "Other Error",
    }
}

#[post("/checkout", data = "<checkout_form>")]
fn order(conn: DbConn, mut checkout_form: Json<models::CheckoutForm>) {
    checkout_form.products.sort_by(|a, b| a.0.cmp(&b.0));

    let payment_amount = database::get_products_price(&conn, &checkout_form.products);

    let order_id =
        database::insert_and_get_order(&conn, &checkout_form.address, &checkout_form.phonenumber);

    database::insert_and_get_payment(
        &conn,
        payment_amount,
        order_id.unwrap(),
        checkout_form.user_id,
    );

    database::insert_products_with_order(&conn, &checkout_form.products, order_id.unwrap());
}

#[get("/product?<pid>")]
fn single(conn: DbConn, pid: i32) -> Template {
    let mut context = Context::new();
    let product = database::get_product(&conn, pid);
    context.insert("title", &format!("{}, - Rusty Shop", product.name));
    context.insert("product", &product);
    context.insert("page", "single");
    Template::render("single", &context)
}

pub fn run_rocket() {
    rocket::ignite()
        .mount(
            "/",
            routes![index, dashboard, checkout, signup, order, single],
        )
        .mount(
            "/assets",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/files")),
        )
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .launch();
}
