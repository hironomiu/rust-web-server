use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddParams {
    text: String,
}

// TODO curlでのcors確認用、不要になったら削除する
// #[head("/")]
pub async fn index_head() -> Result<HttpResponse, actix_web::Error> {
    println!("head /");
    let response_body = "Hello world!!!";
    Ok(HttpResponse::Ok().body(response_body))
}

// #[post("/")]
pub async fn index_post(parms: web::Form<AddParams>) -> Result<HttpResponse, actix_web::Error> {
    println!("post /: {}", parms.text);
    Ok(HttpResponse::Ok().body(String::from(&parms.text)))
}
