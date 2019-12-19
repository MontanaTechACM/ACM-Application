use crate::schema::passwords;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;

#[table_name = "passwords"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Password {
    pub password_id: Option<i32>,
    pub password: String,
    pub verification_code: String,
}

impl Password {
    pub fn create(mut password: Password, connection: &MysqlConnection) -> Password {
        password.password = crate::models::seed_new_password(password.password);
        diesel::insert_into(passwords::table)
            .values(&password)
            .execute(connection)
            .expect("Error creating new password");
        passwords::table.order(passwords::password_id.desc()).first(connection).unwrap()
    }

    pub fn update(password_id: i32, password: Password, connection: &MysqlConnection) -> bool {
        diesel::update(passwords::table.find(password_id)).set(&password).execute(connection).is_ok()
    }

    pub fn delete(password_id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(passwords::table.find(password_id)).execute(connection).is_ok()
    }

    pub fn get_by_password_id(password_id: i32, connection: &MysqlConnection) -> Option<Password> {
        let statement = passwords::table.filter(passwords::password_id.eq(password_id));
        let password = statement.load::<Password>(connection);
        match password {
            Ok(mut password) => password.pop(),
            Err(_) => None,
        }
    }
}