use std::{error::Error, net::SocketAddr};

mod challenge;
mod schema;

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use challenge::Challenge;
use deadpool_diesel::{
    mysql::{Manager, Pool},
    Runtime::Tokio1,
};
use diesel::RunQueryDsl;
use minify_html_onepass::{truncate, Cfg};
use schema::challenges;
use tera::{Context, Tera};

#[derive(Clone)]
struct AppState {
    tera_engine: Tera,
    db_pool: Pool,
}

#[tokio::main]
async fn main() {
    let tera_engine = Tera::new("templates/*").unwrap();

    let manager = Manager::new("mysql://root:root@127.0.0.1/test", Tokio1);
    let db_pool = Pool::builder(manager).build().unwrap();

    let app_state = AppState {
        tera_engine,
        db_pool,
    };

    // build our application with some routes
    let app = Router::new()
        .route("/", get(index))
        .route("/challenges", get(get_challenges))
        .with_state(app_state);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(
    State(AppState {
        tera_engine,
        db_pool: _,
    }): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut ctx = tera::Context::new();
    ctx.insert("foo", "bar");
    render_template(tera_engine, "index.html.tera", &ctx)
        .map_err(internal_error)
}

async fn get_challenges(
    State(AppState {
        tera_engine,
        db_pool,
    }): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = db_pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|f| challenges::table.load::<Challenge>(f))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    
    let mut ctx = tera::Context::new();
    ctx.insert("challenges", &res);

    render_template(tera_engine, "challenges.html.tera", &ctx)
        .map_err(internal_error)
}

fn internal_error<T>(_: T) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

fn render_template(
    tera_engine: Tera,
    template: &str,
    ctx: &Context,
) -> Result<Html<Vec<u8>>, anyhow::Error> {
    let mut buffer = Vec::new();
    tera_engine.render_to(&template, &ctx, &mut buffer)?;

    // Only compiles when optimized
    // Minimizes the template output
    if !cfg!(debug_assertions) {
        let cfg = &Cfg {
            minify_css: true,
            minify_js: true,
        };
        truncate(&mut buffer, cfg)?;
    }

    return Ok(Html(buffer));
}
