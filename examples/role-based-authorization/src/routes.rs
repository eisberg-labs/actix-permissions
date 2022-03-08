use actix_permissions::with;
use actix_web::web::ServiceConfig;
use actix_web::*;

use crate::models::Role;
use crate::permissions::*;

async fn administrators_index() -> Result<String, Error> {
    Ok("Only for administrators!".to_string())
}

async fn moderators_index() -> Result<String, Error> {
    Ok("Only for administrators and moderators!".to_string())
}

async fn index() -> Result<String, Error> {
    Ok("For logged in users!".to_string())
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.route(
        "/",
        check(web::get(), with(has_min_role(Role::User)), index),
    )
    .route(
        "/admin",
        check(
            web::get(),
            with(has_min_role(Role::Administrator)),
            administrators_index,
        ),
    )
    .route(
        "/mod",
        check(
            web::get(),
            with(has_min_role(Role::Moderator)),
            moderators_index,
        ),
    );
}
