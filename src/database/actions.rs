use diesel::prelude::*;
// use uuid::Uuid;
use crate::database::models;
use chrono::NaiveDate;

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Run query using Diesel to find user by uid and return it.
pub fn find_student_by_id(
    conn: &mut MysqlConnection,
    sid: u32,
) -> Result<Option<models::Student>, DbError> {
    use crate::database::schema::student::dsl::*;

    let istudent = student.find(sid).first::<models::Student>(conn).optional()?;

    Ok(istudent)
}

pub fn get_user(conn: &mut MysqlConnection, nm: String) -> Result<Option<models::User>, DbError> {
    use crate::database::schema::user::dsl::*;
    let iuser = user
        .filter(alias.eq(nm))
        .first::<models::User>(conn)
        .optional()?;
    Ok(iuser)
}
/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_student(
    conn: &mut MysqlConnection,
    sdni: i32,
    nm: &String,
    snm: &String,
) -> Result<u32, DbError> {
    use crate::database::schema::student::dsl::*;
    let new_student = models::NewStudent {
        dni: sdni.to_owned(),
        name: nm.to_owned(),
        surname: snm.to_owned(),
    };
    
    diesel::insert_into(student)
        .values(new_student)
        .execute(conn)?;
    let last_inserted_id: u32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Unsigned<diesel::sql_types::Integer>>("LAST_INSERT_ID()"))
        .get_result(conn)?;

    Ok(last_inserted_id)
}

pub fn insert_new_class(
    conn: &mut MysqlConnection,
    a: Option<String>,
    sub: String,
    yd: String,
    dts: (String, String),
) -> Result<u32, DbError> {
    use crate::database::schema::class::dsl::*;
    let new_class = models::NewClass{
        area: a,
        subject: sub,
        year_div: yd,
        date_start: NaiveDate::parse_from_str(dts.0.as_str(), "%d-%m-%Y").unwrap(),
        date_end: NaiveDate::parse_from_str(dts.1.as_str(), "%d-%m-%Y").unwrap(),
    };
    
    diesel::insert_into(class)
        .values(new_class)
        .execute(conn)?;
    let last_inserted_id: u32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Unsigned<diesel::sql_types::Integer>>("LAST_INSERT_ID()"))
        .get_result(conn)?;

    Ok(last_inserted_id)
}

pub fn insert_class_day(
    conn: &mut MysqlConnection,
    day: i8,
    id_class: Option<u32>
){
}
