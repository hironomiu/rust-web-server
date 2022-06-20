use crate::database;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use mysql::prelude::Queryable;
use pwhash::bcrypt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    nickname: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostUser {
    nickname: String,
    email: String,
    password: String,
}

// post
// TODO: SignUpとして移動する
pub async fn index_post(parms: web::Json<PostUser>) -> Result<HttpResponse, actix_web::Error> {
    println!("post /api/v1/users => {},{}", parms.nickname, parms.email);
    let mut conn = database::database();

    let ret = conn
        .exec_map(
            "insert into users(nickname,email,password) values(?,?,?)",
            (
                String::from(&parms.nickname),
                String::from(&parms.email),
                String::from(bcrypt::hash(&parms.password).unwrap()),
            ),
            |(nickname, email, password)| PostUser {
                nickname,
                email,
                password,
            },
        )
        .map_err(|_| HttpResponse::InternalServerError());

    // TODO: 戻り値
    match ret {
        Ok(v) => {
            println!("Ok");
            v
        }
        Err(_) => panic!("error"),
    };
    Ok(HttpResponse::Ok().json(parms))
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
