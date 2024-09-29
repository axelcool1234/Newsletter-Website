//! Newsletter Website
//!
//! This crate provides a simple HTTP server for newsletters built with Actix Web.
//!
//! ## Usage
//! To use this crate, initialize a `TcpListener` and call the `run` function to start the server:
//!
//! ```no_run
//! use std::net::TcpListener;
//! use newsletter_website::run;
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let listener = TcpListener::bind("127.0.0.1:8080")?;
//!     run(listener)?.await
//! }
//! ```
//!
//! ## Endpoints
//! - **GET /health_check**: Used to verify that the server is up and running. Returns `200 OK`.
//! - **POST /subscriptions**: Accepts form data (`email` and `name`) as a subscription request. Returns `200 OK`.
//!
//! ## Dependencies
//! - `actix-web`: Used for handling HTTP server and routing.
//! - `serde`: For deserializing form data in the subscription endpoint.
//! - `tokio`: Provides the asynchronous runtime that Actix Web depends on.
//!
//! ## Tokio Integration
//! This crate uses Tokio as the asynchronous runtime. The `#[tokio::main]` macro is required in the
//! main function to run asynchronous operations. Actix Web internally uses Tokio to handle
//! concurrency and async tasks efficiently.
//!
//! ## Example
//! You can run the server and check the health status by sending a `GET` request to `/health_check`:
//!
//! ```sh
//! curl -X GET http://127.0.0.1:8080/health_check
//! ```
//!
//! Similarly, you can subscribe by sending a `POST` request to `/subscriptions` with form data
//! (in the application/x-www-form-urlencoded format):
//!
//! ```sh
//! curl -X POST http://127.0.0.1:8080/subscriptions -d "name=User&email=user%40example.com"
//! ```
#![warn(missing_debug_implementations, rust_2018_compatibility, missing_docs)]

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

/// Struct representing the form data for subscribing to the service.
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

/// Health check endpoint handler.
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

/// Subscription endpoint handler.
///
/// Due to actix calling the `from_request` method on each argument of all handlers, we can
/// directly take a `FormData` struct rather than a raw HTTPRequest
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}

/// Starts the HTTP server using the given `TcpListener`.
///
/// This function initializes the Actix web server and sets up the routing for our various
/// endpoints.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
