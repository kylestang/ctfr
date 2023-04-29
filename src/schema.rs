// @generated automatically by Diesel CLI.

diesel::table! {
    challenges (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    challenges,
    users,
);
