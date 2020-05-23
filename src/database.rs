use crate::models::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

pub fn get_all_products(connection: &SqliteConnection, limit: i64) -> Vec<Product> {
    use crate::schema::product::dsl::*;

    let results = product
        .limit(limit)
        .load::<Product>(connection)
        .expect("Error loading products");
    results
}

pub fn get_all_customers(connection: &SqliteConnection) {
    use crate::models::*;
    use crate::schema::user::dsl::*;

    let results = user
        .filter(access.eq(3))
        .load::<User>(connection)
        .expect("Error loading users");

    println!("Displaying {} user", results.len());
    for usr in results {
        println!("{:?}", usr);
    }
}
