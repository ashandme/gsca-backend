use serde::{Deserialize, Serialize};
use database::scheme;

#[derive(Queryable, Insertable)]
#[table_name = "student"]
pub struct Student<'a> {
    pub fingerprint: Option<i32>,
    pub dni: i32,
    pub name: &'a str,
    pub surname: &'a str,
}
