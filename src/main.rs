#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, NamedFile};
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

#[get("/")]
fn index() -> Template {
    let context = context! {foo: "World"};
    Template::render("index", &context)
}

#[derive(Serialize)]
struct Challenge {
    name: String,
}

#[derive(Serialize)]
struct Thing {
    challenges: Vec<Challenge>,
}

#[get("/challenges")]
fn challenges() -> Template {
    let context = Thing {
        challenges: vec![
            Challenge {
                name: "Super silly security".to_string(),
            },
            Challenge {
                name: "Smoke on the horizon".to_string(),
            },
            Challenge {
                name: "Smoke on the horizon".to_string(),
            },
            Challenge {
                name: "Smoke on the horizon".to_string(),
            },
            Challenge {
                name: "Super silly security".to_string(),
            },
            Challenge {
                name: "Smoke on the horizon".to_string(),
            },
            Challenge {
                name: "Smoke on the horizon".to_string(),
            },
            Challenge {
                name: "Smoke on the horizon".to_string(),
            },
        ],
    };
    Template::render("challenges", &context)
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/favicon.ico").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, favicon, challenges])
        .mount("/public", FileServer::from("static"))
        .attach(Template::fairing())
}
