use crate::schema::events;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use chrono::NaiveDateTime;

#[table_name = "events"]
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
pub struct Event {
    pub event_id: Option<i32>,
    pub coordinator_id: Option<i32>,
    pub event_type_id: i8,
    pub name: String,
    pub additional_info: Option<String>,
    pub location: String,
    pub event_time: NaiveDateTime,
}

impl Event {
    pub fn create(event: Event, connection: &MysqlConnection) -> Event {
        diesel::insert_into(events::table)
            .values(&event)
            .execute(connection)
            .expect("Error creating new event");
        events::table.order(events::event_id.desc()).first(connection).unwrap()
    }

    pub fn read_all(connection: &MysqlConnection) -> Vec<Event> {
        events::table.order(events::event_id.asc()).load::<Event>(connection).unwrap()
    }

    pub fn update(event_id: i32, event: Event, connection: &MysqlConnection) -> bool {
        diesel::update(events::table.find(event_id)).set(&event).execute(connection).is_ok()
    }

    pub fn delete(event_id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(events::table.find(event_id)).execute(connection).is_ok()
    }
}