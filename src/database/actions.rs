use diesel::prelude::*;
use uuid::Uuid;
use crate::database::models;
type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Run query using Diesel to find user by uid and return it.
/*pub fn find_user_by_uid(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Option<models::User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}
*/

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
        id_student: 0,
	id_fingerprint: Some(0),
	dni: sdni.to_owned(), 
        name: nm.to_owned(),
	surname: snm.to_owned(),
    };

    diesel::insert_into(student).values(&new_student).execute(conn)?;

    Ok(new_student)
}
