use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;

use crate::{
    domain::{Host, HostCategory, Room},
    infrastructure::RoomRepositoryImpl,
    services::get_all_rooms_for_hotel,
};

#[tracing::instrument(name = "Get list of hosts")]
#[get("/hosts")]
pub async fn list_hosts(pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    match get_rooms(&pool).await {
        Ok(_rooms) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::Ok().finish()),
    }
}

// #[tracing::instrument(name = "Get subscriber_id from token", skip())]
#[tracing::instrument(name = "Query all rooms in database")]
pub async fn get_rooms(pool: &PgPool) -> Result<Vec<Room>, String> {
    let hotel_id = 1;
    let room_repo = RoomRepositoryImpl::new();
    let rooms = get_all_rooms_for_hotel(hotel_id, room_repo).await?.unwrap();

    Ok(rooms)
}
