use actix_web::{error, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}};
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
mod database;
use crate::database::{schema::student, models::*, actions::*};
type DbPool = Pool<ConnectionManager<MysqlConnection>>;

#[post("/student")]
async fn add_student(
    pool: web::Data<DbPool>,
    form: web::Json<NewStudent>,
) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let student = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
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
        .expect("database URL should be valid path to SQLite DB file")
}
