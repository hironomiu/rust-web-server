use crate::database;
use actix_web::HttpResponse;
use mysql::prelude::Queryable;
use thiserror::Error;

use serde::Deserialize;
use serde::Serialize;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    nickname: String,
    email: String,
}

// get
pub async fn index_get() -> Result<HttpResponse, actix_web::Error> {
    let mut conn = database::database();
    let ret = conn
        .query_map(
            "select id,nickname,email from users where id = 1",
            |(id, nickname, email)| User {
                id,
                nickname,
                email,
            },
        )
        .map_err(|_| HttpResponse::InternalServerError());

    let mut data = vec![];

    match ret {
        Ok(n) => {
            for i in n {
                data.push(User {
                    id: i.id,
                    nickname: i.nickname,
                    email: i.email,
                });
            }
        }
        Err(_) => println!("Error"),
    }

    println!("get /api/v1/users");
    Ok(HttpResponse::Ok().json(data))
}
