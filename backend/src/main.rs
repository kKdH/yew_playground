/*
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
*/
use axum::body::{boxed, Body};
use axum::http::{Response, StatusCode};
use axum::{response::IntoResponse, routing::get, Router};
use clap::Parser;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "localhost")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    let app = Router::new()
        .route("/api/hello", get(hello))
        .fallback_service(get(|req| async move {
            match ServeDir::new(opt.static_dir).oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    println!("listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}

async fn hello() -> impl IntoResponse {
    "hello from server!"
}

/*
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


 */