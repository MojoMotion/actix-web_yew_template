use actix_web::{
    web::{self, Json},
    App, HttpServer, get, middleware::Logger,
    //http,
};
use actix_files::Files;
//use actix_cors::Cors;

use types::HelloResponse;

const PORT: &str = std::env!("PORT");
//const TRUNK_SERVE_EXTERNAL_PORT: &str = std::env!("TRUNK_SERVE_EXTERNAL_PORT");
//const TRUNK_SERVE_HOST: &str = std::env!("TRUNK_SERVE_HOST");


/**
 * Sample service
 */
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> Json<HelloResponse> {
    Json(HelloResponse {
        name: name.to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

      HttpServer::new(move || {
          let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(greet)
            .service(Files::new("/", "front-end/dist/").index_file("index.html"))
    })
    .bind(("0.0.0.0", PORT.parse::<u16>().unwrap()))?
    .run()
    .await
}













//use actix_web::{web, App, HttpServer, HttpResponse, Responder};
//use actix_files::Files;
//
//async fn greet() -> impl Responder {
//    format!("Hello World!")
//}
//
//#[actix_web::main] // or #[tokio::main]
//async fn main() -> std::io::Result<()> {
//    println!("Start server on 127.0.0.1:8080");
//
//    HttpServer::new(move || {
//        App::new()
//            .service(Files::new("/", "./dist/").index_file("index.html"))
//            .default_service(
//                web::route().to(|| HttpResponse::Found().header("Location", "/").finish()),
//            )
//            .route("/", web::get().to(greet))
//    })
//    .bind(("127.0.0.1", 8080))?
//    .run()
//    .await
//}


