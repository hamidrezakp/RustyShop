table! {
    order (id) {
        id -> Integer,
        datetime -> Text,
        address -> Text,
        phone -> Text,
        status -> Text,
    }
}

table! {
    ordered_products (order_id, product_id) {
        order_id -> Integer,
        product_id -> Integer,
    }
}

table! {
    payment (id) {
        id -> Integer,
        datetime -> Text,
        amount -> Float,
        order_id -> Integer,
        customer_id -> Integer,
    }
}

table! {
    product (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        image -> Text,
        quantity -> Integer,
        price -> Float,
    }
}

table! {
    user (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        firstname -> Text,
        lastname -> Text,
        access -> Integer,
    }
}

joinable!(ordered_products -> order (order_id));
joinable!(ordered_products -> product (product_id));
joinable!(payment -> order (order_id));
joinable!(payment -> user (customer_id));

allow_tables_to_appear_in_same_query!(order, ordered_products, payment, product, user,);
