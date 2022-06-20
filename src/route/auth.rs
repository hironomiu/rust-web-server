use crate::database;
use actix_identity::Identity;
use actix_web::*;
use mysql::prelude::Queryable;
use pwhash::unix;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Auth {
  email: String,
  password: String,
}

// TODO: 認証（SignIn）の実装
pub async fn index_signin_post(
  id: Identity,
  parms: web::Json<Auth>,
) -> Result<HttpResponse, actix_web::Error> {
  println!("post /api/v1/auth/signin");

  let mut conn = database::database();
  let ret = conn
    .exec_map(
      "select email, password from users where email = ?",
      (String::from(&parms.email),),
      |(email, password)| Auth { email, password },
    )
    .map_err(|_| HttpResponse::InternalServerError());
  match ret {
    Ok(n) => {
      println!("{}", n.len());
      // TODO: SELECTにヒットしなかった場合の処理(else)の追加
      if n.len() > 0 {
        // TODO: パスワードハッシュ突き合わせ後の処理
        println!(
          "password is {}",
          unix::verify(&parms.password, &n[0].password)
        );
      }
    }
    Err(_) => panic!("error"),
  };

  // TODO: Identityについて調べる
  id.remember("User1".to_owned());
  // let aa = id.identity();
  // match aa {
  //   Some(v) => println!("v is {}", v),
  //   None => println!("None"),
  // }
  // Ok(HttpResponse::Ok().finish())
  Ok(HttpResponse::Ok().json(parms))
}

pub async fn index_signout_post(id: Identity) -> Result<HttpResponse, actix_web::Error> {
  let aa = id.identity();
  match aa {
    Some(v) => println!("v is {}", v),
    None => println!("None"),
  }
  id.forget();
  Ok(HttpResponse::Ok().json("OK"))
}
