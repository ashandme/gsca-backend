use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

use crate::database::schema::{student, user, class};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = student)]
pub struct Student {
    pub id: u32,
    pub id_fingerprint: Option<u32>,
    pub dni: i32,
    pub name: String,
    pub surname: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone, Queryable)]
#[diesel(table_name = student)]
pub struct NewStudent {
    pub dni: i32,
    pub name: String,
    pub surname: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = user)]
pub struct User {
    pub id: u32,
    pub dni: i32,
    pub secret: String,
    pub email: String,
    pub rol: String,
    pub alias: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub dni: i32,
    pub secret: String,
    pub email: String,
    pub alias: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogUser {
    pub alias: String,
    pub secret: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = class)]
pub struct Class {
    pub id: u32,
    pub area: Option<String>,
    pub subject: String,
    pub year_div: String,
    pub date_start: NaiveDate,
    pub date_end: NaiveDate,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Queryable)]
#[diesel(table_name = class)]
pub struct NewClass {
    pub area: Option<String>,
    pub subject: String,
    pub year_div: String,
    pub date_start: NaiveDate,
    pub date_end: NaiveDate,
}
