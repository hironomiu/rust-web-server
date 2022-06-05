use crate::database;
use actix_web::{web, HttpResponse};
use mysql::prelude::Queryable;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Hello {
  id: Option<u32>,
  col1: String,
  col2: String,
  col3: String,
}
// #[get("/api/v1/hello")]
pub async fn hello_get() -> Result<HttpResponse, actix_web::Error> {
  println!("get /api/v1/hello");
  let mut conn = database::database();
  let ret = conn
    .query_map(
      "select id,col1,col2,col3 from hello",
      |(id, col1, col2, col3)| Hello {
        id,
        col1,
        col2,
        col3,
      },
    )
    .map_err(|_| HttpResponse::InternalServerError());

  let mut data = vec![];
  match ret {
    Ok(n) => {
      for d in n {
        data.push(Hello {
          id: d.id,
          col1: d.col1,
          col2: d.col2,
          col3: d.col3,
        })
      }
    }
    Err(_) => println!("Error!"),
  };

  Ok(HttpResponse::Ok().json(data))
}

#[derive(Serialize, Deserialize)]
pub struct HelloPost {
  col1: String,
  col2: String,
  col3: String,
}

// #[post("/api/v1/hello")]
pub async fn hello_post(parms: web::Json<HelloPost>) -> Result<HttpResponse, actix_web::Error> {
  println!("post /api/v1/hello");
  let id: Option<u32> = Some(1);
  Ok(HttpResponse::Ok().json(Hello {
    id: id,
    col1: String::from(&parms.col1),
    col2: String::from(&parms.col2),
    col3: String::from(&parms.col3),
  }))
}
