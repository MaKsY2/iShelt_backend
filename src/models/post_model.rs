use diesel::Queryable;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, Debug)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user_id: i32,
    pub img_url: String,
}
