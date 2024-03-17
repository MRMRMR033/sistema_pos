// @generated automatically by Diesel CLI.

diesel::table! {
    cash_inflow (inflow_id) {
        inflow_id -> Int4,
        amount -> Nullable<Numeric>,
        description -> Nullable<Text>,
        user_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
    }
}

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
        quantity -> Numeric,
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

diesel::joinable!(cash_inflow -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cash_inflow,
    posts,
    products,
    users,
);
