use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::{launch, post, routes};
mod app_structs;

use app_structs::*;

#[post("/login", data = "<login>")]
fn login(login: Json<Login<'_>>) -> Value {
    json!({ "status": "ok" })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![login])
}
