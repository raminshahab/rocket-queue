use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use crate::schema::users; // assuming the schema module is in `src/schema.rs`

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}