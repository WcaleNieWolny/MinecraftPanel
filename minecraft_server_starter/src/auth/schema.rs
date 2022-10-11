// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
        user_type -> Smallint,
    }
}

// CREATE TABLE USERS(
//     ID INT PRIMARY KEY      NOT NULL,
//     USERNAME      TEXT      NOT NULL,
//     PASSWORD      TEXT      NOT NULL,
//     USER_TYPE     SMALLINT  NOT NULL
// )