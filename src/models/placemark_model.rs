use diesel::Queryable;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize)]
#[diesel(table_name = placemarks)]
pub struct Placemark {
    pub post_id: i32,
    pub lat: f64,
    pub lon: f64,
}
