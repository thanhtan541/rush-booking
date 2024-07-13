use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct BodyData {
    name: String,
    description: String,
    number_of_beds: u8,
}

#[tracing::instrument(
    name = "Add a new room"
    skip(pool, body),
)]
#[post("/rooms")]
pub async fn add_rooms(
    body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let BodyData {
        name,
        description,
        number_of_beds,
    } = body.0;
    Ok(HttpResponse::Ok().finish())
}
