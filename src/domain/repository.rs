use super::Room;

pub trait RoomRepository {
    async fn find_all(&self, hotel_id: u16) -> Result<Option<Vec<Room>>, String>;
}
