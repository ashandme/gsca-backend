use diesel::prelude::*;
use crate::database::models;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Run query using Diesel to find user by uid and return it.
pub fn find_student_by_id(
    conn: &mut MysqlConnection,
    sid: u32,
) -> Result<Option<models::Student>, DbError> {
    use crate::database::schema::student::dsl::*;

    let istudent = student
        .find(sid)
        .first::<models::Student>(conn)
        .optional()?;

    Ok(istudent)
}

pub fn get_all_students(
    conn: &mut MysqlConnection,
) -> Result<Option<Vec<models::Student>>, DbError> {
    use crate::database::schema::student::dsl::*;
    let vstudents = student.select(models::Student::as_select()).load(conn).optional()?;
    Ok (vstudents)
}
pub fn find_class_by_id(
    conn: &mut MysqlConnection,
    sid: u32,
) -> Result<Option<models::Class>, DbError> {
    use crate::database::schema::class::dsl::*;

    let iclass = class.find(sid).first::<models::Class>(conn).optional()?;

    Ok(iclass)
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
    b: &String,
    t: &String,
    nm: &String,
    snm: &String,
) -> Result<u32, DbError> {
    use crate::database::schema::student::dsl::*;
    let new_student = models::NewStudent {
        dni: sdni.to_owned(),
        born: NaiveDate::parse_from_str(b.as_str(), "%d-%m-%Y").unwrap(),
        tel: t.to_owned(),
        name: nm.to_owned(),
        surname: snm.to_owned(),
    };

    diesel::insert_into(student)
        .values(new_student)
        .execute(conn)?;
    let last_inserted_id: u32 = diesel::select(diesel::dsl::sql::<
        diesel::sql_types::Unsigned<diesel::sql_types::Integer>,
    >("LAST_INSERT_ID()"))
    .get_result(conn)?;

    Ok(last_inserted_id)
}

pub fn insert_new_class(
    conn: &mut MysqlConnection,
    a: Option<&String>,
    sub: &String,
    yd: &String,
    dts: (&String, &String),
) -> Result<u32, DbError> {
    use crate::database::schema::class::dsl::*;
    let new_class = models::NewClass {
        area: a.cloned(),
        subject: sub.to_string(),
        year_div: yd.to_string(),
        date_start: NaiveDate::parse_from_str(dts.0.as_str(), "%d-%m-%Y").unwrap(),
        date_end: NaiveDate::parse_from_str(dts.1.as_str(), "%d-%m-%Y").unwrap(),
    };

    diesel::insert_into(class).values(new_class).execute(conn)?;
    let last_inserted_id: u32 = diesel::select(diesel::dsl::sql::<
        diesel::sql_types::Unsigned<diesel::sql_types::Integer>,
    >("LAST_INSERT_ID()"))
    .get_result(conn)?;

    Ok(last_inserted_id)
}

pub fn insert_new_class_day(
    conn: &mut MysqlConnection,
    d: i8,
    class: u32,
    tts: (&String, &String),
) -> Result<u32, DbError> {
    use crate::database::schema::class_day::dsl::*;
    let new_class_day = models::NewClassDay {
        day: d,
        id_class: class,
        time_out: NaiveTime::parse_from_str(tts.0.as_str(), "%H:%M:%S").unwrap(),
        time_in: NaiveTime::parse_from_str(tts.1.as_str(), "%H:%M:%S").unwrap(),
    };

    diesel::insert_into(class_day)
        .values(new_class_day)
        .execute(conn)?;
    let last_inserted_id: u32 = diesel::select(diesel::dsl::sql::<
        diesel::sql_types::Unsigned<diesel::sql_types::Integer>,
    >("LAST_INSERT_ID()"))
    .get_result(conn)?;

    Ok(last_inserted_id)
}

pub fn insert_new_reg(
    conn: &mut MysqlConnection,
    s: u32,
    cd: u32,
    c: Option<&String>,
    tts: (&String, &String),
) -> Result<u32, DbError> {
    use crate::database::schema::reg::dsl::*;
    let new_reg = models::NewReg {
        id_student: s,
        class_day: cd,
        caption: c.cloned(),
        time_out: NaiveDateTime::parse_from_str(tts.0.as_str(), "%d-%m-%Y %H:%M:%S").unwrap(),
        time_in: NaiveDateTime::parse_from_str(tts.1.as_str(), "%d-%m-%Y %H:%M:%S").unwrap(),
    };

    diesel::insert_into(reg).values(new_reg).execute(conn)?;
    let last_inserted_id: u32 = diesel::select(diesel::dsl::sql::<
        diesel::sql_types::Unsigned<diesel::sql_types::Integer>,
    >("LAST_INSERT_ID()"))
    .get_result(conn)?;

    Ok(last_inserted_id)
}

pub fn insert_class_student(
    conn: &mut MysqlConnection,
    s: u32,
    c: u32,
) -> Result<(u32,u32), DbError> {
    use crate::database::schema::class_student::dsl::*;
    let new_cs = models::ClassStudent {
        id_student: s,
        id_class: c,
    };

    diesel::insert_into(class_student).values(new_cs).execute(conn)?;
    Ok((s,c))
}

pub fn insert_prof_class(
    conn: &mut MysqlConnection,
    c: u32,
    u: u32,
) -> Result<(u32,u32), DbError> {
    use crate::database::schema::prof_class::dsl::*;
    let new_prof = models::ProfClass {
        id_class: c,
        id_user: u,
    };

    diesel::insert_into(prof_class).values(new_prof).execute(conn)?;
    Ok((c,u))
}
