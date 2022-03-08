use std::fmt::Debug;
use actix_web::{App, HttpServer, ResponseError, HttpMessage};
use actix_web::web;
use actix_web::http::StatusCode;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;

use role_based_authorization_example::routes::routes;
use thiserror::Error;
use role_based_authorization_example::models::User;

#[derive(Debug, Error)]
pub enum ValidatorError {
    #[error("Forbidden")]
    Forbidden
}

impl ResponseError for ValidatorError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Forbidden => StatusCode::FORBIDDEN,
        }
    }
}

async fn validator(req: ServiceRequest, credentials: BasicAuth) -> Result<ServiceRequest, actix_web::Error> {
    let users = User::list();
    let user = users.iter().find(|it|
        credentials.user_id().eq(&it.username) &&
            credentials.password().is_some() &&
            credentials.password().unwrap().eq(&it.password));
    if let Some(user) = user {
        req.extensions_mut().insert(user.role);
        return Ok(req);
    }

    Err(ValidatorError::Forbidden.into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let auth = HttpAuthentication::basic(validator);
        App::new()
            .wrap(auth)
            .service(web::scope("").configure(routes))
    }).bind("127.0.0.1:8888")?.run().await
}
