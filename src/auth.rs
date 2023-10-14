use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    error,
    HttpMessage as _,
    http::StatusCode,
    middleware, web, App, HttpRequest, HttpServer, Responder,
};

const ONE_MINUTE: Duration = Duration::minutes(1);

/* async fn index(identity: Option<Identity>) -> actix_web::Result<impl Responder> {
    let id = match identity.map(|id| id.id()) {
        None => "anonymous".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };

    Ok(format!("Hello {id}"))
}*/

pub async fn login(req: HttpRequest) -> impl Responder {
    Identity::login(&req.extensions(), "user1".to_owned()).unwrap();

    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}

pub async fn logout(id: Identity) -> impl Responder {
    id.logout();

    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}
