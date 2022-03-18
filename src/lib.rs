// use std::future::ready;
// use std::sync::Arc;
//
// use actix_web::dev::{fn_factory, Payload};
// use actix_web::http::StatusCode;
// use actix_web::{FromRequest, Handler, HttpRequest, HttpResponse, Responder, Route};
//
// use crate::builder::Builder;
// use crate::permission::Permission;
// use crate::service::PermissionService;
//
pub mod builder;
pub mod permission;
pub(crate) mod service;
mod tests;

//
// /// Creates a permission builder, initiated with single permission.
// ///
// /// # Arguments
// /// * `permission` - permission
// pub fn with<P>(permission: P) -> Builder
// where
//     P: Permission + 'static,
// {
//     Builder::new().and(permission)
// }
//
// fn default_deny_handler(_req: &HttpRequest, _payload: &mut Payload) -> HttpResponse {
//     HttpResponse::new(StatusCode::FORBIDDEN)
// }
//
// /// Creates a route which:
// /// - intercepts requests and validates inputs.
// /// - if permission checks are all true, passes through to handler.
// /// - if any of the permissions is false, FORBIDDEN is returned.
// pub fn check<F, Args>(route: Route, builder: Builder, handler: F) -> Route
// where
//     F: Handler<Args>,
//     Args: FromRequest + 'static,
//     F::Output: Responder,
// {
//     let new_perms = Arc::new(builder.permissions);
//     route.service(fn_factory(move || {
//         let new_perms_c = Arc::clone(&new_perms);
//         let handler = handler.clone();
//         ready(Ok(PermissionService::new(
//             new_perms_c,
//             handler,
//             default_deny_handler,
//         )))
//     }))
// }
//
// /// Creates a more flexible route than `check`, which:
// /// - intercepts requests and validates inputs.
// /// - if permission checks are all true, passes through to handler.
// /// - if any of the permissions is false, `deny_handler` is called.
// pub fn check_with_custom_deny<F, Args>(
//     route: Route,
//     builder: Builder,
//     handler: F,
//     deny_handler: fn(&HttpRequest, &mut Payload) -> HttpResponse,
// ) -> Route
// where
//     F: Handler<Args>,
//     Args: FromRequest + 'static,
//     F::Output: Responder,
// {
//     let new_perms = Arc::new(builder.permissions);
//     route.service(fn_factory(move || {
//         let new_perms_c = Arc::clone(&new_perms);
//         let handler = handler.clone();
//         ready(Ok(PermissionService::new(
//             new_perms_c,
//             handler,
//             deny_handler,
//         )))
//     }))
// }
