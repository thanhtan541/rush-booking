mod admin;

use actix_web::{get, HttpResponse};
pub use admin::*;

#[get("/health_check")]
pub async fn health_check() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}
