use rocket::serde::{json::Json, Deserialize};
#[macro_use]
extern crate diesel;
use diesel::{table, Insertable, Queryable};

// @generated automatically by Diesel CLI.

diesel::table! {
    account (user_id) {
        user_id -> Int4,
        user_name -> Varchar,
        user_password -> Varchar,
    }
}
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Login<'r> {
    pub name: &'r str,
    pub password: &'r str,
}
