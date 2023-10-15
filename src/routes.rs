extern crate bcrypt;

use actix_web::{error, web, http::StatusCode, HttpResponse, Responder, HttpRequest, HttpMessage as _};
use actix_identity::Identity;
use bcrypt::{DEFAULT_COST, hash, verify};

use crate::database::{schema::student, models::*, actions::*, DbPool};

pub async fn is_logged(identity: Option<Identity>) -> actix_web::Result<impl Responder> {
    let id = match identity.map(|id| id.id()) {
        None => "anonymous".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };

    Ok(format!("Hello {id} you are logged"))
}

pub async fn login(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    form: web::Json<LogUser>,
) -> actix_web::Result<impl Responder> {
    let log_user = form.into_inner();
    let fuser = web::block(move || {
	let mut conn = pool.get()?;
	get_user(&mut conn, log_user.alias)
    }).await?
	.map_err(error::ErrorInternalServerError)?;
    match fuser {
	Some(fuser) => {
	    let v = verify(&log_user.secret.as_str(), fuser.secret.as_str()).unwrap();
	    match v {
		true => {
		    Identity::login(&req.extensions(), fuser.alias.as_str().to_owned()).unwrap();
		    return Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND))
		}
		false => {
		    return Ok(web::Redirect::to("/").using_status_code(StatusCode::NOT_FOUND))
		}
		_ => {
		    return Err(error::ErrorNotFound("bad password"))
		}
	    };	
	}
	None => Err(error::ErrorInternalServerError("500")),
    }
}

pub async fn logout(id: Identity) -> impl Responder {
    id.logout();
    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}
pub async fn get_student(
    pool: web::Data<DbPool>,
    pid: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    //let user_uid = user_uid.into_inner();
    let id = pid.into_inner();
    let fstudent = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        find_student_by_id(&mut conn, id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match fstudent {
        Some(fstudent) => HttpResponse::Ok().json(fstudent),

        None => HttpResponse::NotFound().body(format!("No user found with ID: {id}")),
    })
}

pub async fn add_student(
    pool: web::Data<DbPool>,
    identity: Option<Identity>,
    form: web::Json<NewStudent>,
) -> actix_web::Result<impl Responder> {
    let student = web::block(move || {
        let mut conn = pool.get()?;
        insert_new_student(&mut conn, form.dni, &form.name, &form.surname)
    })
    .await?
    // map diesel query errors to a 500 error response
	.map_err(error::ErrorInternalServerError)?;
    // user was added successfully; return 201 response with new user info
    Ok(HttpResponse::Created().json(student))
}
