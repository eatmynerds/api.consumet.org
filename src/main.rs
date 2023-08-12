mod models;
mod routes;
mod utils;
use axum::{http::StatusCode, routing::get, Json, Router};
use std::str::FromStr;
use log::info;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    utils::logger::initialize();

    let mut port = std::env::var("PORT").unwrap();
    port = if port.is_empty() {
        "8080".to_string()
    } else {
        port
    };

    // build our application with a route
    let app = Router::new()
        .route("/", get(home))
        .nest("/movies", routes::movies::mount().await)
        .fallback(fallback_func);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = std::net::SocketAddr::from_str(&format!("0.0.0.0:{}", port)).unwrap();
    info!("Server started on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Welcome to consumet api rust! ")
}

async fn fallback_func() -> (StatusCode, Json<models::ResponseError>) {
    (
        StatusCode::NOT_FOUND,
        Json(models::ResponseError {
            message: String::new(),
            error: String::from("page not found"),
        }),
    )
}
