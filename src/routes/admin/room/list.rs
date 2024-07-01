use actix_web::{get, HttpResponse};

#[get("/rooms")]
pub async fn list_rooms() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}
