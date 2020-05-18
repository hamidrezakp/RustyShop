table! {
    Order (id) {
        id -> Integer,
        datetime -> Text,
        address -> Text,
        phone -> Text,
        status -> Text,
    }
}

table! {
    Payment (id) {
        id -> Integer,
        datetime -> Text,
        amount -> Float,
        order -> Integer,
        customer -> Integer,
    }
}

table! {
    Product (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        image -> Text,
        quantity -> Integer,
        price -> Float,
    }
}

table! {
    User (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        firstname -> Text,
        lastname -> Text,
        access -> Integer,
    }
}

table! {
    ordered_products (order, product) {
        order -> Integer,
        product -> Integer,
    }
}

joinable!(Order -> Product (datetime));
joinable!(Payment -> Order (order));
joinable!(Payment -> User (customer));
joinable!(ordered_products -> Order (order));
joinable!(ordered_products -> Product (product));

allow_tables_to_appear_in_same_query!(
    Order,
    Payment,
    Product,
    User,
    ordered_products,
);
