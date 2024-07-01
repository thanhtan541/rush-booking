use std::u8;

// Host is just a meta data of room, not really imnportant
pub struct Host {
    pub category: HostCategory,
    pub name: String,
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

struct Room {
    pub container: Host,
    pub description: String,
    // Assumption that total beds is small
    pub number_of_beds: u8,
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
