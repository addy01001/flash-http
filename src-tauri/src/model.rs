use diesel::{prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::histories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct History {
    pub id: i32,
    pub url: String,
    pub body: String,
    pub headers: String,
    pub method: String,
    pub created_at: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::histories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewHistory {
    pub url: String,
    pub method: String,
    pub body: String,
    pub headers: String
}