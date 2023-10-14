use jsonwebtoken::{decode, DecodingKey, Validation};
use actix_service::Service;
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use std::pin::Pin;
use futures::future::{ok, Ready};

pub struct JwtMiddleware;

impl<S, B> Transform<S> for JwtMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = JwtMiddlewareTransform<S>;
    type InitError = ();
    fn new_transform(&self, service: S) -> Self::Transform {
        JwtMiddlewareTransform { service }
    }
}

pub struct JwtMiddlewareTransform<S> {
    service: S,
}

impl<S, B> Service for JwtMiddlewareTransform<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // Aquí verificas el JWT en el encabezado de la solicitud
        // Si es válido, permites que continúe, de lo contrario, respondes con un error
        Box::pin(async { Ok(req.into_response(HttpResponse::Unauthorized())) })
    }
}

#[post("/login")]
async fn login() -> HttpResponse {
    // Aquí debes verificar las credenciales y generar el JWT si son válidas
    // Por ejemplo:
    let token = encode(...)?;
    HttpResponse::Ok().json(token)
}
