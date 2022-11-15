use rocket::serde::{json::Json, Deserialize};
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Login<'r> {
    name: &'r str,
    password: &'r str,
}
