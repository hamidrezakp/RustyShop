use crate::models::*;
use crate::schema;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::QueryResult;
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

pub fn insert_and_get_payment(
    connection: &SqliteConnection,
    payment_amount: f32,
    order: i32,
    user: i32,
) -> Option<i32> {
    use schema::payments::dsl::*;

    let result = insert_into(payments)
        .values((
            datetime.eq(Utc::now().naive_utc()),
            amount.eq(payment_amount),
            order_id.eq(order),
            user_id.eq(user),
        ))
        .execute(connection);

    if let Ok(1) = result {
        Some(
            payments
                .select(id)
                .order(id.desc())
                .limit(1)
                .load::<i32>(connection)
                .expect("Error loading Payments id")[0],
        )
    } else {
        None
    }
}

pub fn insert_and_get_order(
    connection: &SqliteConnection,
    in_address: &String,
    phonenumber: &String,
) -> Option<i32> {
    use schema::orders::dsl::*;

    let result = insert_into(orders)
        .values((
            datetime.eq(Utc::now().naive_utc()),
            address.eq(in_address),
            phone.eq(phonenumber),
        ))
        .execute(connection);

    if let Ok(1) = result {
        Some(
            orders
                .select(id)
                .order(id.desc())
                .limit(1)
                .load::<i32>(connection)
                .expect("Error loading Orders id")[0],
        )
    } else {
        None
    }
}

pub fn get_products_price(connection: &SqliteConnection, product_pairs: &Vec<(i32, usize)>) -> f32 {
    use schema::products::dsl::*;

    let product_ids: Vec<i32> = product_pairs.iter().map(|pair| pair.0).collect();

    let mut products_prices = products
        .select((id, price))
        .filter(id.eq_any(product_ids.clone()))
        .load::<(i32, f32)>(connection)
        .expect("Error loading Products");

    products_prices.sort_by(|a, b| a.0.cmp(&b.0));

    product_pairs
        .iter()
        .zip(products_prices)
        .map(|item| (item.0).1 as f32 * (item.1).1)
        .sum::<f32>()
}

pub fn insert_products_with_order(
    connection: &SqliteConnection,
    in_products: &Vec<(i32, usize)>,
    in_order_id: i32,
) {
    use schema::ordered_products::dsl::*;

    let ordered_products_pair: Vec<OrderedProduct> = in_products
        .iter()
        .map(|item| OrderedProduct {
            order_id: in_order_id,
            product_id: (item.0),
        })
        .collect();

    insert_into(ordered_products)
        .values(ordered_products_pair)
        .execute(connection)
        .unwrap();
}
