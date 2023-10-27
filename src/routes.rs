extern crate bcrypt;

use actix_identity::Identity;
use actix_web::{
    error, http::StatusCode, web, HttpMessage as _, HttpRequest, HttpResponse, Responder,
};
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::database::{actions::*, models::*, schema::student, DbPool};

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
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    match fuser {
        Some(fuser) => {
            let v = verify(&log_user.secret.as_str(), fuser.secret.as_str()).unwrap();
            match v {
                true => {
                    Identity::login(&req.extensions(), fuser.alias.as_str().to_owned()).unwrap();
                    return Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND));
                }
                false => return Ok(web::Redirect::to("/").using_status_code(StatusCode::NOT_FOUND)),
                _ => return Err(error::ErrorNotFound("bad password")),
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
    pid: web::Path<u32>,
) -> actix_web::Result<impl Responder> {
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

pub async fn get_students(
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let vstudent = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        get_all_students(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match vstudent {
        Some(v) => HttpResponse::Ok().json(v),

        None => HttpResponse::NotFound().body(format!("SORRY")),
    })
}
pub async fn get_class(
    pool: web::Data<DbPool>,
    pid: web::Path<u32>,
) -> actix_web::Result<impl Responder> {
    let id = pid.into_inner();
    let fclass = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        find_class_by_id(&mut conn, id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match fclass {
        Some(fclass) => HttpResponse::Ok().json(fclass),

        None => HttpResponse::NotFound().body(format!("No user found with ID: {id}")),
    })
}

pub async fn add_student(
    pool: web::Data<DbPool>,
    identity: Option<Identity>,
    form: web::Json<JsonStudent>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().unwrap();
    match insert_new_student(&mut conn, form.dni, &form.born, &form.tel, &form.name, &form.surname) {
        Ok(x) => Ok(HttpResponse::Created().json(x)),
        Err(e) => Err(error::ErrorInternalServerError(e)),
    }
}

pub async fn add_class(
    pool: web::Data<DbPool>,
    identity: Option<Identity>,
    form: web::Json<JsonClass>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().unwrap();
    match insert_new_class(
        &mut conn,
        Some(&form.area),
        &form.subject,
        &form.year_div,
        (&form.time_out, &form.time_in),
    ) {
        Ok(x) => Ok(HttpResponse::Created().json(x)),
        Err(e) => Err(error::ErrorInternalServerError(e)),
    }
}

pub async fn add_class_day(
    pool: web::Data<DbPool>,
    identity: Option<Identity>,
    form: web::Json<JsonClassDay>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().unwrap();
    match insert_new_class_day(
        &mut conn,
        form.day,
        form.id_class,
        (&form.time_out, &form.time_in),
    ) {
        Ok(x) => Ok(HttpResponse::Created().json(x)),
        Err(e) => Err(error::ErrorInternalServerError(e)),
    }
}

pub async fn add_reg(
    pool: web::Data<DbPool>,
    identity: Option<Identity>,
    form: web::Json<JsonReg>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().unwrap();
    match insert_new_reg(
        &mut conn,
        form.id_student,
        form.class_day,
        Some(&form.caption),
        (&form.time_out, &form.time_in),
    ) {
        Ok(x) => Ok(HttpResponse::Created().json(x)),
        Err(e) => Err(error::ErrorInternalServerError(e)),
    }
}

pub async fn add_class_student(
    pool: web::Data<DbPool>,
    identity: Option<Identity>,
    form: web::Json<ClassStudent>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().unwrap();
    match insert_class_student(
        &mut conn,
        form.id_student,
        form.id_class,
    ) {
        Ok(x) => Ok(HttpResponse::Created().json(x)),
        Err(e) => Err(error::ErrorInternalServerError(e)),
    }
}

pub async fn create_user(
    pool: web::Data<DbPool>,
    identity: Option<Identity>,
    form: web::Json<NewUser>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().unwrap();
    match insert_new_user(
        &mut conn,
        form.dni,
        &form.secret,
        &form.email,
        &form.tel,
        &form.rol,
        &form.alias,
    ) {
        Ok(x) => Ok(HttpResponse::Created().json(x)),
        Err(e) => Err(error::ErrorInternalServerError(e)),
    }
}
