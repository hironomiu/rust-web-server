use actix_cors::Cors;
use actix_web::{get, head, http, post, web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
struct AddParams {
    text: String,
}

#[get("/")]
async fn index_get() -> Result<HttpResponse, actix_web::Error> {
    println!("get /");
    let response_body = "Hello world!!!";
    Ok(HttpResponse::Ok().body(response_body))
}

// TODO curlでのcors確認用、不要になったら削除する
#[head("/")]
async fn index_head() -> Result<HttpResponse, actix_web::Error> {
    println!("head /");
    let response_body = "Hello world!!!";
    Ok(HttpResponse::Ok().body(response_body))
}

#[post("/")]
async fn index_post(parms: web::Form<AddParams>) -> Result<HttpResponse, actix_web::Error> {
    println!("post /: {}", parms.text);
    Ok(HttpResponse::Ok().body("body"))
}

#[derive(Serialize, Deserialize)]
struct Hello {
    id: Option<u32>,
    message: String,
    is_success: bool,
}

#[get("/api/v1/hello")]
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

#[post("/api/v1/hello")]
async fn hello_post(parms: web::Json<HelloPost>) -> Result<HttpResponse, actix_web::Error> {
    println!("post /api/v1/hello {}", parms.message);
    let message = "hello!hello!";
    let id: Option<u32> = Some(1);
    Ok(HttpResponse::Ok().json(Hello {
        id: id,
        message: String::from(message),
        // message: String::from(parms.message),
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
            .service(index_get)
            .service(index_head)
            .service(index_post)
            .service(hello_get)
            .service(hello_post)
    })
    .bind("localhost:5555")?
    .run()
    .await?;
    Ok(())
}
