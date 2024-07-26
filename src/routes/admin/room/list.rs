use actix_web::{get, http::header::ContentType, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{GeneralName, Host, HostCategory, Room},
    infrastructure::RoomRepositoryImpl,
    services::get_all_rooms_for_hotel,
    utils::ResponseData,
};

#[tracing::instrument(name = "Get list of rooms")]
#[get("/rooms")]
pub async fn list_rooms(pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let host = Host {
        id: Uuid::new_v4(),
        name: GeneralName::parse("Intercontinental".into()).unwrap(),
        category: HostCategory::parse("hotel").unwrap(),
    };
    let rooms = vec![Room {
        id: Uuid::new_v4(),
        name: GeneralName::parse("Standard 2-bed rooms".into()).unwrap(),
        container: host,
        description: "Two beds room".into(),
        number_of_beds: 2,
    }];

    let response = ResponseData {
        data: rooms,
        code: 200,
        message: format!("Successfully retrieving data"),
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response))
}

#[tracing::instrument(name = "Query all rooms in database")]
pub async fn get_rooms(pool: &PgPool) -> Result<Vec<Room>, String> {
    let hotel_id = 1;
    let room_repo = RoomRepositoryImpl::new();
    let rooms = get_all_rooms_for_hotel(hotel_id, room_repo).await?.unwrap();

    Ok(rooms)
}
