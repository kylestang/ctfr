#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::Debug;
use rocket_dyn_templates::{context, Template};
use rocket_sync_db_pools::database;

use crate::challenge::Challenge;
use crate::schema::*;

mod challenge;
mod schema;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[database("db")]
struct Db(diesel::MysqlConnection);

#[get("/")]
fn index() -> Template {
    let context = context! {foo: "World"};
    Template::render("index", &context)
}

#[get("/challenges")]
async fn get_challenges(db: Db) -> Result<Template> {
    let chals = db
        .run(|conn| challenges::table.load::<Challenge>(conn))
        .await?;
    Ok(Template::render("challenges", context!(challenges: chals)))
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/favicon.ico").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, favicon, get_challenges])
        .mount("/public", FileServer::from("static"))
        .attach(Template::fairing())
        .attach(Db::fairing())
}
