use axum::{routing::get, Router};

mod routes;

use routes::day_minus_one;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(day_minus_one));

    Ok(router.into())
}
