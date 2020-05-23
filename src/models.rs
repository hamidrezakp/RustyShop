use crate::schema::*;
use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Draft,
    Sent,
    Error(String),
}

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "order"]
pub struct Order {
    pub id: i32,
    pub datetime: NaiveDateTime,
    pub address: String,
    pub phone: String,
    pub status: Status,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[belongs_to(Order)]
#[table_name = "payment"]
pub struct Payment {
    pub id: i32,
    pub datetime: NaiveDateTime,
    pub amount: f32,
    pub order_id: i32,
    pub user_id: i32,
}

#[derive(Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[belongs_to(Order)]
#[belongs_to(Product)]
#[table_name = "ordered_products"]
pub struct OrderedProduct {
    order_id: i32,
    product_id: i32,
}

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "product"]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub quantity: i32,
    pub price: f32,
}

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub access: i32,
}
