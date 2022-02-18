// extern crate mysql;
// extern crate r2d2;
// extern crate r2d2_mysql;

mod database;
mod route;

use actix_cors::Cors;
use actix_web::{
    // get, post, head,
    http,
    web,
    App,
    HttpResponse,
    HttpServer,
    ResponseError,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}
impl ResponseError for MyError {}

use serde::Deserialize;
use serde::Serialize;

// dotenv
use dotenv::dotenv;
use std::env;

// DB
// use mysql::prelude::Queryable;

#[derive(Serialize)]
struct RootEntry {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct Hello {
    id: Option<u32>,
    message: String,
    is_success: bool,
}

// #[get("/api/v1/hello")]
async fn hello_get() -> Result<HttpResponse, actix_web::Error> {
    println!("get /api/v1/hello");
    let message = "hello!hello!";
    let id: Option<u32> = Some(1);

    let ret = [
        Hello {
            id: id,
            message: String::from(message),
            is_success: true,
        },
        Hello {
            id: id,
            message: String::from(message),
            is_success: true,
        },
    ];

    Ok(HttpResponse::Ok().json(ret))
}

#[derive(Serialize, Deserialize)]
struct HelloPost {
    message: String,
}

// #[post("/api/v1/hello")]
async fn hello_post(parms: web::Json<HelloPost>) -> Result<HttpResponse, actix_web::Error> {
    println!("post /api/v1/hello {}", parms.message);
    let id: Option<u32> = Some(1);
    Ok(HttpResponse::Ok().json(Hello {
        id: id,
        message: String::from(&parms.message),
        is_success: true,
    }))
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
            // .service(index_post)
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/v1")
                            // /api/v1/hello
                            // .service(web::resource("/hello").to(hello_get))
                            .route("/hello", web::get().to(hello_get))
                            .route("/hello", web::post().to(hello_post))
                            //
                            .route("/users", web::get().to(route::users::index_get)),
                    )
                    // /api/hello
                    .route("/hello", web::get().to(hello_get)),
            )
    })
    .bind(server_address)?
    .run()
    .await?;
    Ok(())
}
