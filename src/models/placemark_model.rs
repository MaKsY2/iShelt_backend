use diesel::{prelude::Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};

use crate::schema::placemarks;

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = placemarks)]
pub struct Placemark {
    pub post_id: i32,
    pub lat: f64,
    pub lon: f64,
}
