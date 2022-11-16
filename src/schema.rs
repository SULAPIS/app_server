// @generated automatically by Diesel CLI.

diesel::table! {
    account (user_id) {
        user_id -> Int4,
        user_name -> Varchar,
        user_password -> Varchar,
    }
}
