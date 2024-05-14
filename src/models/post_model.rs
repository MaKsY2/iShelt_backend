use diesel::{prelude::Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};

use crate::schema::posts;

#[derive(Queryable, Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user_id: Option<i32>,
    pub img_url: Option<String>,
}
