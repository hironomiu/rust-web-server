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
  println!("called");
  // TODO: Identityについて調べる
  id.remember("User1".to_owned());
  Ok(HttpResponse::Ok().json(parms))
}
