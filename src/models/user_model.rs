use diesel::Queryable;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub name: String,
    pub email: String,
    pub id: i32,
}
