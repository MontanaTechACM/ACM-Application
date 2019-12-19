use crate::schema::user_types;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;

#[table_name = "user_types"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Usertype {
    pub user_type_id: Option<i8>,
    pub name: String,
    pub description: String,
}

impl Usertype {
    pub fn read(user_type_id: i8, connection: &MysqlConnection) -> Option<Usertype> {
        let statement = user_types::table.filter(user_types::user_type_id.eq(user_type_id));
        let user = statement.load::<Usertype>(connection);
        match user {
            Ok(mut user) => user.pop(),
            Err(_) => None,
        }
    }
}