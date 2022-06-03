use crate::database;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use mysql::prelude::Queryable;
use serde::Deserialize;
use serde::Serialize;

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
            "select id,nickname,email from users",
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

    println!("get /api/v1/users/");
    Ok(HttpResponse::Ok().json(data))
}

pub async fn index_id_get(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();
    let mut conn = database::database();
    let ret = conn
        .exec_map(
            "select id,nickname,email from users where id = ?",
            // TODO: Request時にPATHでuser.idを受けて指定する（このエンドポイントはusers全体を返す）
            (userid,),
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

    println!("get /api/v1/users/{}", userid);
    Ok(HttpResponse::Ok().json(data))
}
