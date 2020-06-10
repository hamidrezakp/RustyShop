table! {
    ordered_products (id) {
        id -> Integer,
        order_id -> Integer,
        product_id -> Integer,
        quantity -> Integer,
    }
}

table! {
    orders (id) {
        id -> Integer,
        datetime -> Timestamp,
        address -> Text,
        phone -> Text,
        status -> Text,
    }
}

table! {
    payments (id) {
        id -> Integer,
        datetime -> Timestamp,
        amount -> Float,
        order_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    products (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        image -> Text,
        quantity -> Integer,
        price -> Float,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        firstname -> Text,
        lastname -> Text,
        access -> Integer,
    }
}

joinable!(ordered_products -> orders (order_id));
joinable!(ordered_products -> products (product_id));
joinable!(payments -> orders (order_id));
joinable!(payments -> users (user_id));

allow_tables_to_appear_in_same_query!(ordered_products, orders, payments, products, users,);
