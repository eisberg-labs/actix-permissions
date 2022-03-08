use actix_permissions::permission::Permission;
use actix_web::dev::Payload;
use actix_web::{HttpMessage, HttpRequest};
use std::future::{ready, Ready};
use crate::models::Role;

#[derive(Clone)]
pub struct RolePermissionCheck {
    role: Role,
}

impl Permission for RolePermissionCheck {
    fn call(&self, req: &HttpRequest, _payload: &mut Payload) -> Ready<actix_web::Result<bool>> {
        let is_allowed = req.extensions().get::<Role>().map(|user_role| self.role >= *user_role).unwrap_or(false);
        let res: actix_web::Result<bool, actix_web::Error> = Ok(is_allowed);
        ready(res)
    }
}

/// Returns true if logged in user's role is equal or higher than role
pub fn has_min_role(role: Role) -> RolePermissionCheck {
    RolePermissionCheck { role }
}
