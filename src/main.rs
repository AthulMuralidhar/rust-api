#[macro_use]
extern crate actix_web;

use std::{env, io};
use actix_web::{middleware, App, HttpServer};



#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(||{
        App::new()
            .wrap(middleware::Logger::default())
            .service()
    }).bind("0.0.0.0:8000")?.run().await;
}