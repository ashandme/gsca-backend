use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    error, get,
    middleware::{Logger, NormalizePath},
    post, web, App, HttpResponse, HttpServer, Responder,
};
use diesel::{
    mysql::MysqlConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenv::dotenv;

mod database;
mod routes;

use crate::database::{actions::*, models::*, schema::student, DbPool};

const ONE_MINUTE: Duration = Duration::minutes(1);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = initialize_db_pool();

    log::info!("starting HTTP server at http://localhost:8080");
    let secret_key = Key::generate();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/login").route(web::post().to(routes::login)))
            .service(web::resource("/logout").route(web::post().to(routes::logout)))
            .service(web::resource("/").route(web::get().to(routes::is_logged)))
            .service(
                web::resource("/student/{id_student}").route(web::get().to(routes::get_student)),
            )
            .service(web::resource("/student").route(web::post().to(routes::add_student)))
            .service(web::resource("/class").route(web::post().to(routes::add_class)))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("auth-token".to_owned())
                    .cookie_secure(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(ONE_MINUTE))
                    .build(),
            )
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
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
