use actix_web::{get, web, HttpResponse};
use sqlx::{types::Uuid, PgPool};

use crate::domain::{Host, HostCategory, Room};

#[tracing::instrument(name = "Get list of rooms")]
#[get("/rooms")]
pub async fn list_rooms(pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    match get_rooms(&pool).await {
        Ok(_rooms) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::Ok().finish()),
    }
}

// #[tracing::instrument(name = "Get subscriber_id from token", skip())]
#[tracing::instrument(name = "Query all rooms in database")]
pub async fn get_rooms(pool: &PgPool) -> Result<Vec<Room>, String> {
    let rooms = vec![Room {
        container: Host {
            category: HostCategory::parse("hotel").expect("Invalid hotel category"),
            name: "InterContinental".to_string(),
        },
        number_of_beds: 2,
        description: "Double beds".to_string(),
    }];

    Ok(rooms)
}

#[tracing::instrument(name = "Get subscriber_id from token", skip(pool))]
pub async fn get_subscriber_id_from_token(pool: &PgPool) -> Result<Option<String>, sqlx::Error> {
    let result = sqlx::query!("SELECT name FROM hosts",)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(result.map(|r| r.name))
}
