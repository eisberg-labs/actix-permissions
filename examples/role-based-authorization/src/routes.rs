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
        permission()
            .check(web::get())
            .validate(has_min_role(Role::User))
            .to(index)
            .build(),
    )
    .route(
        "/admin",
        permission()
            .check(web::get())
            .validate(has_min_role(Role::Administrator))
            .to(administrators_index)
            .build(),
    )
    .route(
        "/mod",
        permission()
            .check(web::get())
            .validate(has_min_role(Role::Moderator))
            .to(moderators_index)
            .build(),
    );
}
