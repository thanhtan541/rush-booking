use std::marker::PhantomData;

use uuid::Uuid;

trait BookingState {}

#[derive(Debug)]
struct UserId(String);

#[derive(Debug)]
pub struct Session<State: BookingState = Initial> {
    pub id: String,
    pub user_id: Option<UserId>,
    pub marker: PhantomData<State>,
}

impl Session {
    pub fn new() -> Session<Initial> {
        let id = Uuid::new_v4().to_string();
        Session::<Initial> {
            id,
            user_id: None,
            marker: PhantomData,
        }
    }
}

impl Session<Initial> {
    pub fn checkin(self) -> Session<Ongoing> {
        Session::<Ongoing> {
            id: self.id,
            user_id: Some(UserId("test".into())),
            marker: PhantomData,
        }
    }
}

impl Session<Ongoing> {
    pub fn finish(self) -> Session<Done> {
        Session::<Done> {
            id: self.id,
            user_id: self.user_id,
            marker: PhantomData,
        }
    }

    pub fn cancel(self) -> Session<Cancelled> {
        Session::<Cancelled> {
            id: self.id,
            user_id: self.user_id,
            marker: PhantomData,
        }
    }
}

impl Session<Done> {
    pub fn completed(self) -> bool {
        true
    }
}

impl Session<Cancelled> {
    pub fn cancelled(self) -> bool {
        true
    }
}

#[derive(Debug, Default)]
struct Initial;
#[derive(Debug, Default)]
struct Ongoing;
#[derive(Debug, Default)]
struct Cancelled;
#[derive(Debug, Default)]
struct Done;

impl BookingState for Initial {}
impl BookingState for Ongoing {}
impl BookingState for Cancelled {}
impl BookingState for Done {}

#[cfg(test)]
mod test {
    use super::Session;

    #[test]
    fn check_api() {
        let session = Session::new();
        let ongoing_session = session.checkin();
        let done = ongoing_session.finish();

        assert!(done.completed());
    }
}
