use axum::{
    routing::get,
    Router,
};
use axum::response::{Html, IntoResponse};
use axum::{body::Bytes, http::StatusCode};
use axum::extract::{Path, Query};
use axum::routing::get_service;
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());


    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/", get(handler_main))
        .route("/hello/:name", get(handler_hello))
        .route("/foo", get(get_new_page))
        .route("/foo/bar", get(echo))
}

async fn echo(body: Bytes) -> Result<String, StatusCode> {
    if let Ok(string) = String::from_utf8(body.to_vec()) {
        Ok(string)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

async fn get_new_page() -> Html<&'static str> {
    Html(include_str!("../../src/main.rs"))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// localhost:8080/?nam=Jan -> begrüßt Jan
async fn handler_main(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_main - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    format!("Hello {name}, is this working and efficient?")
}

// localhost:8080/hello/Mike -> begrüßt Mike
async fn handler_hello(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_main - {name:?}", "HANDLER");

    format!("Hello {name}, is this working and efficient?")
}
