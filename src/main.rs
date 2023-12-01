#![allow(dead_code)]
extern crate diesel;

/**
 * External crates
 */
use actix_web::{web, App, HttpServer};
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use futures::{select, FutureExt};
use std::error::Error;

use crate::middlewares::auth_middleware::Authentication;
/**
 * Load internal modules
 **/
pub mod api;
pub mod constants;
pub mod error;

pub mod models;
pub mod repositories;
pub mod routers;
pub mod services;
pub mod middlewares;
pub mod utilities;
pub mod schema;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //Defin Log
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let api_host = dotenv::var("API_HOST").unwrap();

    // Redis
    let redis = redis::Client::open("redis://127.0.0.1:6379").unwrap();

    // Create connection for PostgreSQL
    let config =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(std::env::var("DATABASE_URL")?);
    let pool = Pool::builder(config).build()?;

    let mut server_future = HttpServer::new(move || {
        App::new()
            .wrap(Authentication)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis.clone()))
            .configure(routers::app::config_services)
    })
    .bind(api_host)?
    .run()
    .fuse();

    //Http Server Notify
    select! {
        _r = server_future => println!("Server is stopped!"),
    };
    Ok(())
}
