use actix_web::{get,web,post, App, HttpResponse, HttpServer};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
struct AddParams{
    text: String,
}

#[get("/")]
async fn index_get() -> Result<HttpResponse, actix_web::Error> {
    println!("get /");
    let response_body = "Hello world!!!";
    Ok(HttpResponse::Ok().body(response_body))
}

#[post("/")]
async fn index_post(parms: web::Form<AddParams>) -> Result<HttpResponse,actix_web::Error>{
    println!("post /: {}",parms.text);
    Ok(HttpResponse::Ok().body("body"))
}

#[derive(Serialize,Deserialize)]
struct Hello {
    id: Option<u32>,
    message: String,
    is_success: bool,
}

#[get("/api/v1/hello")]
async fn hello_get() -> Result<HttpResponse,actix_web::Error> {
    println!("get /api/v1/hello");
    let message = "hello!hello!";
    let id: Option<u32> = Some(1);
    Ok(HttpResponse::Ok().json(Hello{
        id:id,
        message:String::from(message),
        is_success: true,
    }))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    println!("server create");
    HttpServer::new(move || App::new().service(index_get).service(hello_get).service(index_post))
        .bind("localhost:5555")?
        .run()
        .await?;
    Ok(())
}
