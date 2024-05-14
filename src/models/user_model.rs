use diesel::{prelude::Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};

use crate::schema;

#[derive(Queryable, Deserialize, Serialize)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = schema::users)]
pub struct UserCreateModel {
    pub name: String,
    pub email: String,
}
