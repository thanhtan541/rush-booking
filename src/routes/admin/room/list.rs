use actix_web::{get, HttpResponse};

use crate::domain::{Host, HostCategory, Room};

#[tracing::instrument(name = "Get list of rooms")]
#[get("/rooms")]
pub async fn list_rooms() -> Result<HttpResponse, actix_web::Error> {
    match get_rooms().await {
        Ok(_rooms) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::Ok().finish()),
    }
}

// #[tracing::instrument(name = "Get subscriber_id from token", skip())]
#[tracing::instrument(name = "Query all rooms in database")]
pub async fn get_rooms() -> Result<Vec<Room>, String> {
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
