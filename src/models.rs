use diesel::prelude::*;
use crate::schema::users;
use chrono::{DateTime, Local};

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id : i32,
    pub name : String,
    pub creationdate : DateTime<Local>,
    pub email : String,
    pub password : String,
}
