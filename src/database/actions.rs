use diesel::prelude::*;
// use uuid::Uuid;
use crate::database::models;
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
) -> Result<models::Student, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::database::schema::student::dsl::*;
    // TODO temporal values
    let new_student = models::Student {
        id: 0,
        id_fingerprint: None,
        dni: sdni.to_owned(),
        name: nm.to_owned(),
        surname: snm.to_owned(),
    };

    diesel::insert_into(student)
        .values(&new_student)
        .execute(conn)?;

    Ok(new_student)
}
