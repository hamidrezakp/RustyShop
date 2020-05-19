use crate::models::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_all_products(connection: &SqliteConnection, limit: i64) -> Vec<Product> {
    use crate::schema::product::dsl::*;

    let results = product
        .limit(limit)
        .load::<Product>(connection)
        .expect("Error loading products");
    results
}

pub fn get_all_customers() {
    use crate::models::*;
    use crate::schema::user::dsl::*;

    let connection = establish_connection();
    let results = user
        .filter(access.eq(3))
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} user", results.len());
    for usr in results {
        println!("{:?}", usr);
    }
}
