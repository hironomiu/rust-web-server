use actix_web::{web, HttpResponse, ResponseError};
use serde::Deserialize;
use serde::Serialize;

// DB
// main.rs で mod database で利用できる
use crate::database;
use mysql::prelude::Queryable;

// テンプレートエンジン
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<RootEntry>,
}

// #[derive(Deserialize)]
// pub struct AddParams {
//     text: String,
// }

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}
impl ResponseError for MyError {}

#[derive(Serialize)]
struct RootEntry {
    text: String,
}

// #[get("/")]
pub async fn index_get() -> Result<HttpResponse, MyError> {
    println!("get /");
    let mut entries = Vec::new();
    entries.push(RootEntry {
        text: "こんにちは！".to_string(),
    });
    entries.push(RootEntry {
        text: "hello!".to_string(),
    });

    let mut conn = database::database();

    #[derive(Serialize, Deserialize)]
    pub struct Organization {
        pub id: i32,
        pub name: String,
        pub price: u32,
    }

    let ret = conn
        // 複数カラムの取得例
        .query_map("SELECT id,name,price from items", |(id, name, price)| {
            Organization { id, name, price }
        })
        .map_err(|_| HttpResponse::InternalServerError());
    match ret {
        Ok(n) => {
            for i in n {
                entries.push(RootEntry {
                    text: format!("{} {} {}", i.id, i.name, i.price),
                });
                println!("{} {} {}", i.id, i.name, i.price);
            }
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
pub async fn index_head() -> Result<HttpResponse, actix_web::Error> {
    println!("head /");
    let response_body = "Hello world!!!";
    Ok(HttpResponse::Ok().body(response_body))
}

#[derive(Serialize, Deserialize)]
pub struct Hello {
    col1: String,
    col2: String,
    col3: String,
}

// #[post("/")]
pub async fn index_post(parms: web::Json<Hello>) -> Result<HttpResponse, actix_web::Error> {
    println!("post /");
    let mut conn = database::database();
    // MEMO: insert sample
    let ret = conn
        .exec_map(
            "insert into hello(col1,col2,col3) values(?,?,?)",
            (
                parms.col1.to_string(),
                parms.col2.to_string(),
                parms.col3.to_string(),
            ),
            |(col1, col2, col3)| Hello { col1, col2, col3 },
        )
        .map_err(|_| HttpResponse::InternalServerError());
    // TODO: retの使い道(last insert idの取り方)
    let ret = match ret {
        Ok(v) => v,
        Err(_) => {
            panic!("error");
        }
    };

    Ok(HttpResponse::Ok().json(ret))
}
