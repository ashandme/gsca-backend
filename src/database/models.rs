use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::database::schema::{class, class_day, reg, student, user, class_student, prof_class};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = student)]
pub struct Student {
    pub id: u32,
    pub id_fingerprint: Option<u32>,
    pub dni: i32,
    pub born: NaiveDate,
    pub tel: String,
    pub name: String,
    pub surname: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone, Queryable)]
#[diesel(table_name = student)]
pub struct NewStudent {
    pub dni: i32,
    pub born: NaiveDate,
    pub tel: String,
    pub name: String,
    pub surname: String,
}

#[derive(Serialize, Deserialize)]
pub struct JsonStudent {
    pub dni: i32,
    pub born: String,
    pub tel: String,
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
    pub tel: String,
    pub rol: String,
    pub alias: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub dni: i32,
    pub secret: String,
    pub email: String,
    pub tel: String,
    pub rol: String,
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

#[derive(Serialize, Deserialize)]
pub struct JsonClass {
    pub area: String,
    pub subject: String,
    pub year_div: String,
    pub time_out: String,
    pub time_in: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = class_student)]
pub struct ClassStudent {
    pub id_class: u32,
    pub id_student: u32,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = prof_class)]
pub struct ProfClass {
    pub id_class: u32,
    pub id_user: u32,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = class_day)]
pub struct ClassDay {
    pub id: u32,
    pub day: i8,
    pub id_class: Option<u32>,
    pub time_out: NaiveTime,
    pub time_in: NaiveTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = class_day)]
pub struct NewClassDay {
    pub day: i8,
    pub id_class: u32,
    pub time_out: NaiveTime,
    pub time_in: NaiveTime,
}

#[derive(Serialize, Deserialize)]
pub struct JsonClassDay {
    pub day: i8,
    pub id_class: u32,
    pub time_out: String,
    pub time_in: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = reg)]
pub struct Reg {
    pub id: u32,
    pub id_student: u32,
    pub class_day: u32,
    pub caption: Option<String>,
    pub time_out: NaiveDateTime,
    pub time_in: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = reg)]
pub struct NewReg {
    pub id_student: u32,
    pub class_day: u32,
    pub caption: Option<String>,
    pub time_out: NaiveDateTime,
    pub time_in: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct JsonReg {
    pub id_student: u32,
    pub class_day: u32,
    pub caption: String,
    pub time_out: String,
    pub time_in: String,
}
