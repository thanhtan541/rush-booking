use uuid::Uuid;

use crate::domain::{GeneralName, Host, HostCategory, Room, RoomRepository};

pub struct RoomRepositoryImpl {
    _rooms: Option<Vec<Room>>,
}

impl RoomRepositoryImpl {
    pub fn new() -> Self {
        Self { _rooms: None }
    }
}

impl RoomRepository for RoomRepositoryImpl {
    async fn find_all(&self, _hotel_id: u16) -> Result<Option<Vec<crate::domain::Room>>, String> {
        let rooms = vec![Room {
            container: Host {
                category: HostCategory::parse("hotel").expect("Invalid hotel category"),
                name: GeneralName::parse("Intercontinentel".to_string())?,
                id: Uuid::new_v4(),
            },
            name: GeneralName::parse("Double beds room".to_string())?,
            number_of_beds: 2,
            description: "Double beds".to_string(),
            id: Uuid::new_v4(),
        }];

        Ok(Some(rooms))
    }
}
