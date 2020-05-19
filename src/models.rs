use crate::schema::*;
use chrono::naive::NaiveDateTime;

#[derive(Debug, PartialEq)]
pub enum Status {
    Draft,
    Sent,
    Error(String),
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "order"]
pub struct Order {
    id: i32,
    datetime: NaiveDateTime,
    address: String,
    phone: String,
    status: Status,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Order)]
#[table_name = "payment"]
pub struct Payment {
    id: i32,
    datetime: NaiveDateTime,
    amount: f32,
    order_id: i32,
    user_id: i32,
}

#[derive(Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Order)]
#[belongs_to(Product)]
#[table_name = "ordered_products"]
pub struct OrderedProduct {
    order_id: i32,
    product_id: i32,
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "product"]
pub struct Product {
    id: i32,
    name: String,
    description: String,
    image: String,
    quantity: i32,
    price: f32,
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "user"]
pub struct User {
    id: i32,
    username: String,
    password: String,
    firstname: String,
    lastname: String,
    access: i32,
}
