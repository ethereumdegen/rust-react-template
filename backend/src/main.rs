#[macro_use]
extern crate log;

pub mod db;
mod util;

mod controllers;

//pub mod test;
mod ai_modules;
mod types;

//mod config;

use controllers::chat_controller::ChatController;
use controllers::command_controller::CommandController;
use controllers::oauth_controller::OAuthController;
use controllers::web_controller::WebController;

use degen_sql::db::postgres::postgres_db::{Database, DatabaseCredentials};

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

use std::io;
use std::sync::Arc;

use std::fs;

use dotenvy::dotenv;

// This struct represents state
pub struct AppState {
    database: Arc<Database>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    // Initialize the logger
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info"); // Adjust as per your needs
    env_logger::init();

    println!("connecting to db.");


    fs::create_dir_all("./tmp").unwrap();

    let database_credentials = DatabaseCredentials::from_env();
    
  //  println!({},database_credentials);

    let database = Arc::new(Database::connect(database_credentials, None).await.unwrap());

    let _app_state = Arc::new(AppState {
        database: Arc::clone(&database),
    });

    println!("connected to db.");

    //setup and launch the http server
    HttpServer::new(move || {
        let cors = Cors::default()
            //  .allowed_origin("http://localhost:3000")
            // .allowed_origin("http://localhost:8080")
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Authorization", "Accept", "Content-Type"])
            .supports_credentials()
            .max_age(3600);

        let app_state = AppState {
            database: Arc::clone(&database),
        };

        App::new()
            .app_data(Data::new(app_state)) // Clone your db connection or use Arc
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default()) // Enable logger middleware
            .configure(OAuthController::config)
            .configure(ChatController::config)
            .configure(CommandController::config)
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await
}
