use crate::db::Connection;
use rocket::{self};
use rocket_contrib::json::{Json};
use crate::models::event_type::Eventtype;
use crate::models::event::Event;
use crate::models::user::User;
use rocket_failure::errors::*;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
struct NewEventMedium {
    pub coordinator_id: i32,
    pub event_type_id: i8,
    pub name: String,
    pub additional_info: String,
    pub location: String,
    pub event_time: NaiveDateTime,
}

#[post("/create", data = "<event_medium>")]
fn create(event_medium: Json<NewEventMedium>, connection: Connection) -> Result<Json<Event>, Status> {
    let medium = event_medium.into_inner();
    let val_user: bool = User::validate_user_id(medium.coordinator_id, &connection);
    let val_event_type: bool = Eventtype::validate_event_type_id(medium.event_type_id, &connection);
    if val_user && val_event_type {
        let event = Event {
            event_id: None,
            coordinator_id: Some(medium.coordinator_id),
            event_type_id: medium.event_type_id,
            name: medium.name,
            additional_info: Some(medium.additional_info),
            location: medium.location,
            event_time: medium.event_time,
        };
        let event = Event::create(event, &connection);
        Ok(Json(event))
    } else {
        Err(Status::BadRequest)
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/event", routes![create])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use super::Event;
    use chrono::NaiveDateTime;
    use std::str::FromStr;
    use crate::init_rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn create_event_test_good() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/event", routes![super::create]);
        let client = Client::new(rocket).unwrap();
        let body_event: Event = Event {
            event_id: None,
            coordinator_id: Some(1),
            event_type_id: 1,
            name: String::from("Test event"),
            additional_info: Some(String::from("This is a test event")),
            location: String::from("Dalton\'s house"),
            event_time: NaiveDateTime::from_str("2007-04-05T14:30:30").ok().unwrap(),
        };
        let mut response = client.post("/event/create")
            .body(serde_json::to_string::<Event>(&body_event).ok().unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let event_result: Event = serde_json::from_str::<Event>(response.body_string().unwrap().as_str()).ok().unwrap();
        assert_eq!(event_result.coordinator_id, body_event.coordinator_id);
        assert_eq!(event_result.event_type_id, body_event.event_type_id);
        assert_eq!(event_result.name, body_event.name);
        assert_eq!(event_result.location, body_event.location);
        assert_eq!(event_result.event_time, body_event.event_time);
    }

}