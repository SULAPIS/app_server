#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use crate::diesel::RunQueryDsl;
use diesel::{table, Insertable, Queryable};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::State;
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};
#[database("my_db")]
pub struct Db(diesel::PgConnection);
// @generated automatically by Diesel CLI.

diesel::table! {
    account (user_id) {
        user_id -> Int4,
        user_name -> Varchar,
        user_password -> Varchar,
    }
}
#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[table_name = "account"]
struct Account {
    user_id: i32,
    user_name: String,
    user_password: String,
}

#[get("/random")]
fn get_random_blog_post() -> Json<Account> {
    Json(Account {
        user_id: 1,
        user_name: "steven".to_string(),
        user_password: "universe".to_string(),
    })
}

#[get("/<user_id>")]
fn get_blog_post(user_id: i32) -> Json<Account> {
    Json(Account {
        user_id,
        user_name: "lapis".to_string(),
        user_password: "lazuli".to_string(),
    })
}

#[post("/", data = "<blog_post>")]
async fn create_blog_post(connection: Db, blog_post: Json<Account>) -> Json<Account> {
    connection
        .run(move |c| {
            diesel::insert_into(account::table)
                .values(&blog_post.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("boo")
}
#[get("/")]
async fn get_all_blog_posts(connection: Db) -> Json<Vec<Account>> {
    connection
        .run(|c| account::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog posts")
}
#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();

    rocket
        .attach(Db::fairing())
        .mount("/", routes![get_all_blog_posts, create_blog_post])
        .mount(
            "/blog-posts",
            routes![
                get_random_blog_post,
                get_blog_post,
                get_all_blog_posts,
                create_blog_post
            ],
        )
}
