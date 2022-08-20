#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserializer;
use listenfd::ListenFd;
use dotenv::dotenv;
use std::env;

mod models;
mod routes;
mod api_error;
mod schema;
mod db;

// https://cloudmaker.dev/how-to-create-a-rest-api-in-rust/
// awesome resource!!
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            //User routes
            
            //Player routes
            .configure(routes::player_routes::init_routes)
    });
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");

    server.run().await
}
