use crate::{db::*, endpoints::*, error::*, helper::*, schema::*, structs::*};
use actix_multipart::{Multipart, MultipartError};
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, ResponseError};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
    result::Error as DieselError,
};
use futures::{channel::mpsc::SendError, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::{
    env, fmt,
    fs::File,
    io::{self, Error as StdError, Write},
    path::Path,
    sync::mpsc::{channel, RecvError},
    thread,
};
use uuid::Uuid;

mod db;
mod endpoints;
mod error;
mod helper;
mod schema;
mod structs;

// Database Pool
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Read host usr from .env file
    let args: Vec<String> = env::args().collect();
    let host_url: &str = &args[1];
    let con_str: &str = &args[2];

    // Establish DB Pool
    let pool = establish_db_pool(con_str)?;
    println!("Starting server at: {}", &host_url);

    let data = web::Data::new(pool);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            .service(upload_file)
            .service(get_files)
            .service(download_file)
    })
    .bind(host_url)?
    .run()
    .await
}
