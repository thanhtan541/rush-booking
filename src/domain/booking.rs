use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(Debug)]
pub struct GeneralName(String);

impl GeneralName {
    pub fn parse(s: String) -> Result<GeneralName, String> {
        let is_empty = s.is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid Subscriber's name", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for GeneralName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// Host is just a meta data of room, not really imnportant
pub struct Host {
    pub id: Uuid,
    pub category: HostCategory,
    pub name: GeneralName,
}

pub struct NewHost {
    pub name: GeneralName,
    pub category: HostCategory,
}

// Different kind of romm's container
// It can be belong to a hotel or local house
// But not both
#[derive(Debug)]
pub enum HostCategory {
    Hotel,
    GuestHouse,
}

impl HostCategory {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "hotel" => Ok(HostCategory::Hotel),
            "guest_house" => Ok(HostCategory::GuestHouse),
            _ => Err(format!("{} is not a valid host category!", s)),
        }
    }
}

impl AsRef<str> for HostCategory {
    fn as_ref(&self) -> &str {
        match self {
            HostCategory::GuestHouse => "guest_house",
            HostCategory::Hotel => "hotel",
        }
    }
}

pub struct Room {
    pub id: Uuid,
    pub container: Host,
    pub name: GeneralName,
    pub description: String,
    // Assumption that total beds is small
    pub number_of_beds: u16,
}

pub struct NewRoom {
    pub host_id: Uuid,
    pub name: GeneralName,
    pub description: String,
    // Assumption that total beds is small
    pub number_of_beds: u16,
}

#[cfg(test)]
mod tests {
    use claims::assert_err;

    use super::HostCategory;

    #[test]
    fn invalid_hotel_category_is_rejected() {
        let host_category = "prison";

        assert_err!(HostCategory::parse(host_category));
    }

    #[test]
    fn valid_hotel_category_is_accepted() {
        let host_category = "hotel";
        assert!(matches!(
            HostCategory::parse(host_category).unwrap(),
            HostCategory::Hotel
        ));

        let host_category = "guest_house";
        assert!(matches!(
            HostCategory::parse(host_category).unwrap(),
            HostCategory::GuestHouse
        ));
    }
}
