// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    products (id_product) {
        id_product -> Int4,
        name -> Varchar,
        bar_code -> Varchar,
        cost_price -> Numeric,
        sales_price -> Numeric,
        promotion_price -> Nullable<Numeric>,
        stock -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        active -> Bool,
        pass -> Varchar,
        phone -> Varchar,
        email -> Varchar,
        username -> Varchar,
        lastname -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    products,
    users,
);
