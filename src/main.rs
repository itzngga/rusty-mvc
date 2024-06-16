use actix_web::{middleware, App, HttpServer};
use actix_web_validator::{JsonConfig, PathConfig, QueryConfig};
use api_practice::controller;
use api_practice::exception::error_handler::handle_error;
use dotenv::dotenv;
use std::env;

async fn init() {
    dotenv().ok();
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    init().await;

    let port: u16 = env::var("PORT")
        .expect("missing port")
        .parse()
        .expect("invalid port");

    println!("Server is running on 0.0.0.0:{}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(PathConfig::default().error_handler(handle_error))
            .app_data(QueryConfig::default().error_handler(handle_error))
            .app_data(JsonConfig::default().error_handler(handle_error))
            .configure(controller::user_controller::setup_routes)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
