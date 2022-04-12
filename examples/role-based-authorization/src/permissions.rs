use crate::models::Role;
use actix_permissions::builder::Builder;
use actix_permissions::permission::Permission;
use actix_web::{FromRequest, Handler, HttpMessage, HttpRequest, HttpResponse, Responder};
use std::future::{ready, Ready};

#[derive(Clone)]
pub struct RolePermissionCheck {
    role: Role,
}

fn custom_deny_handler(req: HttpRequest) -> HttpResponse {
    let role_exists = req.extensions().get::<Role>().is_some();
    if !role_exists {
        return HttpResponse::Unauthorized().body("You don't have access rights!");
    } else {
        return HttpResponse::Forbidden().body("Forbidden!");
    }
}

impl Permission<()> for RolePermissionCheck {
    type Future = Ready<actix_web::Result<bool>>;
    fn call(&self, req: HttpRequest, _ctx: ()) -> Ready<actix_web::Result<bool>> {
        let is_allowed = req
            .extensions()
            .get::<Role>()
            .map(|user_role| self.role >= *user_role)
            .unwrap_or(false);
        let res: actix_web::Result<bool> = Ok(is_allowed);
        ready(res)
    }
}

/// Returns true if logged in user's role is equal or higher than role
pub fn has_min_role(role: Role) -> RolePermissionCheck {
    RolePermissionCheck { role }
}

pub fn permission<F, Args, P1, P1Args>() -> Builder<F, Args, P1, P1Args>
where
    F: Handler<Args>,
    Args: FromRequest + 'static,
    P1: Permission<P1Args>,
    P1Args: FromRequest + 'static,
    F::Output: Responder,
{
    actix_permissions::permission().with_deny_handler(custom_deny_handler)
}
