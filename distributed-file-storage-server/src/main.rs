use crate::{db::*, endpoints::*, error::*, helper::*, schema::*, structs::*};
use actix_multipart::{Multipart, MultipartError};
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, ResponseError};
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{self, ConnectionManager},
    result::Error as DieselError,
};
use futures::{channel::mpsc::SendError, StreamExt, TryStreamExt};
use log::info;
use serde::{Deserialize, Serialize};
use std::{
    env, fmt,
    fs::File,
    io::{self, Error as StdError, Write},
    path::{Path, PathBuf},
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
#[actix_web::main] // Entry point of the Actix Web application
async fn main() -> io::Result<()> {
    // Set the log level to "info" (controls the verbosity of logging)
    std::env::set_var("RUST_LOG", "info");

    // Initialize the logger to capture log output based on the RUST_LOG environment variable
    env_logger::init();

    // Retrieve the database connection string from the environment variable "DATABASE_URL"
    // If the variable is not set, the program will panic with a message
    let con_str = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

    // Retrieve the host address from the environment variable "HOST_ADDR"
    // This is the address where the server will bind (e.g., "127.0.0.1:8080")
    let host_addr = env::var("HOST_ADDR").expect("HOST_ADDR environment variable not set");

    // Establish a connection pool to the database using the connection string
    let pool = establish_db_pool(&con_str)?;

    // Wrap the pool in Actix Web's data extractor (shared application state)
    let data = web::Data::new(pool);

    // Start the HTTP server
    HttpServer::new(move || {
        // Create a new Actix Web app instance
        App::new()
            // Pass the database pool to the app's data (so handlers can access it)
            .app_data(data.clone())
            // Enable logging middleware to log incoming requests and responses
            .wrap(middleware::Logger::default())
            // Register the file upload service endpoint
            .service(upload_file)
            // Register the file metadata retrieval service endpoint
            .service(get_files)
            // Register the file download service endpoint
            .service(download_file)
    })
    .bind(&host_addr)? // Bind the server to the provided host address (from "HOST_ADDR" env variable)
    .run() // Run the server (await its completion, it will run until shutdown)
    .await
}
