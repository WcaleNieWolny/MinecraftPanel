// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
        user_type -> Smallint,
    }
}

diesel::table! {
    sessions (id) {
        id -> Nullable<Integer>,
        expiration -> Timestamp,
        user_id -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);