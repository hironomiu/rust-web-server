mod database;
mod route;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}
impl ResponseError for MyError {}

use serde::Serialize;

// dotenv
use dotenv::dotenv;
use std::env;

#[derive(Serialize)]
struct RootEntry {
    text: String,
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    dotenv().ok();
    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set");
    let cors_allowed_origin =
        env::var("CORS_ALLOWED_ORIGIN").expect("CORS_ALLOWED_ORIGIN must be set");
    println!("server create:{}", server_address);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cors_allowed_origin)
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(
                web::scope("/")
                    .route("", web::get().to(route::index::index_get))
                    .route("", web::head().to(route::index::index_head))
                    .route("", web::post().to(route::index::index_post)),
            )
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        // /api/v1/hello
                        .route("/hello", web::get().to(route::hello::hello_get))
                        .route("/hello", web::post().to(route::hello::hello_post))
                        // /api/v1/users
                        .route("/users", web::get().to(route::users::index_get))
                        .route("/users", web::post().to(route::users::index_post))
                        .route(
                            "/users/{user_id}",
                            web::get().to(route::users::index_id_get),
                        )
                        .service(
                            web::scope("/auth")
                                // /api/v1/auth
                                .route("/signin", web::post().to(route::auth::index_signin_post)),
                        ),
                ),
            )
    })
    .bind(server_address)?
    .run()
    .await?;
    Ok(())
}
