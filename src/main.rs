extern crate mysql;
extern crate r2d2;
extern crate r2d2_mysql;

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

// テンプレートエンジン
use askama::Template;

use serde::Deserialize;
use serde::Serialize;

// DB
use mysql::prelude::Queryable;
use mysql::{Opts, OptsBuilder};
use r2d2_mysql::MysqlConnectionManager;
use std::sync::Arc;
// use std::thread;

const DATABASE_HOST: &str = "127.0.0.1";
const DATABASE_PORT: &str = "3306";
const DATABASE_USER: &str = "root";
const DATABASE_PASS: &str = "mysql";
const DATABASE_NAME: &str = "test";
const DATABASE_POOL_SIZE: u32 = 4;

#[derive(Deserialize)]
struct AddParams {
    text: String,
}

#[derive(Serialize)]
struct RootEntry {
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<RootEntry>,
}

// Database Connectionを返す
fn database() -> r2d2::PooledConnection<r2d2_mysql::MysqlConnectionManager> {
    let db_url = format!(
        "mysql://{user}:{pass}@{host}:{port}/{name}",
        user = DATABASE_USER,
        pass = DATABASE_PASS,
        host = DATABASE_HOST,
        port = DATABASE_PORT,
        name = DATABASE_NAME
    );
    let opts = Opts::from_url(&db_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MysqlConnectionManager::new(builder);
    let pool = Arc::new(
        r2d2::Pool::builder()
            .max_size(DATABASE_POOL_SIZE)
            .build(manager)
            .unwrap(),
    );
    let pool = pool.clone();
    let conn = pool.get().unwrap();
    return conn;
}

// #[get("/")]
async fn index_get() -> Result<HttpResponse, MyError> {
    println!("get /");
    let mut entries = Vec::new();
    entries.push(RootEntry {
        text: "こんにちは！".to_string(),
    });
    entries.push(RootEntry {
        text: "hello!".to_string(),
    });

    let mut conn = database();

    #[derive(Serialize, Deserialize)]
    pub struct Organization {
        pub num: i32,
    }

    let ret = conn
        .query_map("SELECT 1 as num", |num| Organization { num })
        .map_err(|_| HttpResponse::InternalServerError());
    match ret {
        Ok(n) => {
            println!("num is {:?}", n[0].num);
            let num = n[0].num;
            entries.push(RootEntry {
                text: num.to_string(),
            });
        }
        Err(_) => println!("Error"),
    }

    let html = IndexTemplate { entries };
    let response_body = html.render()?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

// TODO curlでのcors確認用、不要になったら削除する
// #[head("/")]
async fn index_head() -> Result<HttpResponse, actix_web::Error> {
    println!("head /");
    let response_body = "Hello world!!!";
    Ok(HttpResponse::Ok().body(response_body))
}

// #[post("/")]
async fn index_post(parms: web::Form<AddParams>) -> Result<HttpResponse, actix_web::Error> {
    println!("post /: {}", parms.text);
    Ok(HttpResponse::Ok().body(String::from(&parms.text)))
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
    Ok(HttpResponse::Ok().json(Hello {
        id: id,
        message: String::from(message),
        is_success: true,
    }))
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
    println!("server create");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(
                web::scope("/")
                    .route("", web::get().to(index_get))
                    .route("", web::head().to(index_head))
                    .route("", web::post().to(index_post)),
            )
            // .service(index_post)
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/v1")
                            // /api/v1/hello
                            // .service(web::resource("/hello").to(hello_get))
                            .route("/hello", web::get().to(hello_get))
                            .route("/hello", web::post().to(hello_post)),
                    )
                    // /api/hello
                    .route("/hello", web::get().to(hello_get)),
            )
    })
    // TODO env に出す
    .bind("localhost:5555")?
    .run()
    .await?;
    Ok(())
}
