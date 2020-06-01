use crate::models::*;
use crate::schema;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn get_all_products(connection: &SqliteConnection, limit: i64) -> Vec<Product> {
    use schema::products::dsl::*;

    products
        .limit(limit)
        .load::<Product>(connection)
        .expect("Error loading products")
}

pub fn get_all_customers(connection: &SqliteConnection) -> Vec<User> {
    use schema::users::dsl::*;

    users
        .filter(access.eq(3))
        .load::<User>(connection)
        .expect("Error loading users")
}

pub fn insert_user(connection: &SqliteConnection, in_user: FormUser) {
    use schema::users::dsl::*;

    insert_into(users)
        .values(&in_user)
        .execute(connection)
        .unwrap();
}
