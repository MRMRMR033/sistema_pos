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
    users (id) {
        id -> Int4,
        name -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        cel -> Varchar,
        house_cel -> Varchar,
        active -> Bool,
        admin -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
