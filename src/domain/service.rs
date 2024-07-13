use super::Room;

pub trait RoomService {
    //! Fetch rooms
    //! Todo: add more condition later
    fn fetch_rooms() -> Result<Option<Vec<Room>>, String>;
}
