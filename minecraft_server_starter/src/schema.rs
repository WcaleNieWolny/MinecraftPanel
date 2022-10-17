// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Nullable<Integer>,
        expiration -> Timestamp,
        user_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
        user_type -> SmallInt,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
