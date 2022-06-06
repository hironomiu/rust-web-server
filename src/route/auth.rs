use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Auth {
  email: String,
  password: String,
}

// TODO: 認証（SignIn）の実装
pub async fn index_signin_post(parms: web::Json<Auth>) -> Result<HttpResponse, actix_web::Error> {
  println!("called");

  Ok(HttpResponse::Ok().json(parms))
}
