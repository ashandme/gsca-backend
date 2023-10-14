use actix_web::{error, get , post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}};
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
mod database;
use crate::database::{schema::student, models::*, actions::*};
type DbPool = Pool<ConnectionManager<MysqlConnection>>;

#[get("/student/{studentx_id}")]
async fn get_student(
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
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match fstudent {
        // user was found; return 200 response with JSON formatted user object
        Some(fstudent) => HttpResponse::Ok().json(fstudent),

        // user was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No user found with ID: {id}")),
    })
}

#[post("/student")]
async fn add_student(
    pool: web::Data<DbPool>,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // initialize DB pool outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();

    //log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // add DB pool handle to app data; enables use of `web::Data<DbPool>` extractor
            .app_data(web::Data::new(pool.clone()))
            // add request logger middleware
            .wrap(Logger::default())
        // add route handlers
	    .service(get_student)
            .service(add_student)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = ConnectionManager::<MysqlConnection>::new(conn_spec);
    Pool::builder()
        .build(manager)
        .expect("CAN'T CREATE POOL FROM DATABASE_URL")
}
