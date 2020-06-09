use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "orders"]
pub struct Order {
    pub id: i32,
    pub datetime: NaiveDateTime,
    pub address: String,
    pub phone: String,
    pub status: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct OrderWithPrice {
    pub datetime: NaiveDateTime,
    pub address: String,
    pub phone: String,
    pub status: String,
    pub price: f32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[belongs_to(Order, foreign_key = "order_id")]
#[table_name = "payments"]
pub struct Payment {
    pub id: i32,
    pub datetime: NaiveDateTime,
    pub amount: f32,
    pub order_id: i32,
    pub user_id: i32,
}

#[derive(Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[belongs_to(Order, foreign_key = "order_id")]
#[belongs_to(Product, foreign_key = "product_id")]
#[table_name = "ordered_products"]
pub struct OrderedProduct {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(PartialEq, Debug, Insertable)]
#[table_name = "ordered_products"]
pub struct NewOrderedProduct {
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "products"]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub quantity: i32,
    pub price: f32,
}

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub access: i32,
}

/// The user type we get in Forms
#[derive(Debug, PartialEq, FromForm, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct FormUser {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub password: String,
}

/// The type we get in checkout page
#[derive(Debug, Deserialize)]
pub struct CheckoutForm {
    pub user_id: i32,
    pub products: Vec<(i32, i32)>,
    pub address: String,
    pub phonenumber: String,
}
