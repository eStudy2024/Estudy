// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        display_name -> Varchar,
        password -> Varchar,
        user_role -> Varchar,
    }
}
