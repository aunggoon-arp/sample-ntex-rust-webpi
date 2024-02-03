#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

use std::fs as stdfs;

use controller::{
    admin_controller, auth_controller, file_controller, role_controller, root_controller,
    user_controller,
};
use dotenv::dotenv;
use ntex::{
    http::{self, header},
    web::{self, middleware::Logger, App, HttpServer},
};
use ntex_cors::Cors;
use ntex_files as fs;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
// use utils::swagger_docs::ApiDoc;
// use utoipa::OpenApi;
// use utoipa_swagger_ui::SwaggerUi;

mod config;
mod controller;
mod dto;
mod entity;
mod error;
mod query;
mod service;
mod utils;

pub struct MySqlState {
    pub db: MySqlPool,
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful !");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let dir_creator_image = stdfs::create_dir("./upload/image");
    match dir_creator_image {
        Ok(()) => {}
        Err(_err) => {
            println!("Skip create directory /upload/image, directory is exist.")
        }
    }

    let dir_creator_file = stdfs::create_dir("./upload/file");
    match dir_creator_file {
        Ok(()) => {}
        Err(_err) => {
            println!("Skip create directory /upload/file, directory is exist.")
        }
    }

    println!("Started server at port 8080...");

    // let openapi = ApiDoc::openapi();
    HttpServer::new(move || {
        App::new()
        .state(MySqlState { db: pool.clone() })
        .service(
            web::scope("/api")
                .configure(root_controller::root_route_config)
                .configure(admin_controller::admin_route_config)
                .configure(auth_controller::auth_route_config)
                .configure(file_controller::file_route_config)
                .configure(role_controller::role_route_config)
                .configure(user_controller::user_route_config),
        )
        .service(fs::Files::new("/upload", "./upload").use_last_modified(true))
        .wrap(
            Cors::new()
                .allowed_origin("*")
                .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
                .allowed_headers(vec![
                    header::CONTENT_TYPE,
                    http::header::AUTHORIZATION,
                    http::header::ACCEPT,
                    http::header::ORIGIN,
                    http::header::X_CONTENT_TYPE_OPTIONS,
                ])
                .max_age(3600)
                .finish(),
        )
        // .service(SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
        .wrap(Logger::default())
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
