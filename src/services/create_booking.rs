use crate::domain::{Room, RoomRepository};

pub async fn get_all_rooms_for_hotel(
    hotel_id: u16,
    repo: impl RoomRepository,
) -> Result<Option<Vec<Room>>, String> {
    let rooms = repo.find_all(hotel_id).await?;
    Ok(rooms)
}
