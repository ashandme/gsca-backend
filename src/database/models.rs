use serde::{Deserialize, Serialize,};
use diesel::{Queryable, Insertable,};

use crate::database::schema::student;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = student)]
pub struct Student {
    pub id_student: i32,
    pub id_fingerprint: Option<i32>,
    pub dni: i32,
    pub name: String,
    pub surname: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewStudent {
    pub dni: i32,
    pub name: String,
    pub surname: String,
}
