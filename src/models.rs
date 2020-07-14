use crate::schema::*;
use serde::{ Deserialize, Serialize };
use diesel::sql_types::Timestamp;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Users {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime
}

//struct for auto generation of id
#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

