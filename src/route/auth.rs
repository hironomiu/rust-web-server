use actix_identity::Identity;
use actix_web::*;

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
