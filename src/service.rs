use std::convert::Infallible;
use std::future::Future;
use std::marker::PhantomData;
use std::process::Output;
use std::sync::Arc;

use actix_web::dev::{Payload, Service, ServiceRequest, ServiceResponse};
use actix_web::{dev, FromRequest, Handler, HttpRequest, HttpResponse, Responder};

use crate::permission::{Permission, PinnedFuture};

/// Service that intercepts request, validates it with a list of permissions.
/// If any of the permissions fail, 403 forbidden is returned.
/// If permissions succeed, request is proxied to handler
///
/// # Properties
/// * `perms` - list of permissions
/// * `handler` - handler, a function that returns http (serializable) response
/// * `phantom_data` - phantom data, needed to avoid warnings of unused `Args`
pub struct PermissionService<'r, F, Args>
where
    F: Handler<Args>,
    Args: FromRequest,
    F::Output: Responder,
{
    perms: Arc<Vec<Box<dyn Permission<'r>>>>,
    handler: F,
    phantom_data: PhantomData<Args>,
    deny_handler: fn(&HttpRequest, &mut Payload) -> HttpResponse,
}

impl<'r, F, Args> PermissionService<'r, F, Args>
where
    F: Handler<Args>,
    Args: FromRequest,
    F::Output: Responder,
{
    pub fn new(
        perms: Arc<Vec<Box<dyn Permission<'r>>>>,
        handler: F,
        deny_handler: fn(&HttpRequest, &mut Payload) -> HttpResponse,
    ) -> Self {
        Self {
            perms,
            handler,
            phantom_data: PhantomData::default(),
            deny_handler,
        }
    }
}

impl<'r, F, Args> Service<ServiceRequest> for PermissionService<'r, F, Args>
where
    F: Handler<Args>,
    Args: FromRequest,
    F::Output: Responder,
{
    type Response = ServiceResponse;
    type Error = Infallible;
    type Future = PinnedFuture<'r, Result<Self::Response, Self::Error>>;

    dev::always_ready!();

    fn call(&self, args: ServiceRequest) -> Self::Future {
        let (req, mut payload) = args.into_parts();
        let perms = Arc::clone(&self.perms);
        let handler = self.handler.clone();
        let deny_handler = self.deny_handler;

        Box::pin(async move {
            for permission in perms.iter() {
                match permission.check(&req, &mut payload).await {
                    Ok(false) => {
                        let response = deny_handler(&req, &mut payload);
                        return Ok(ServiceResponse::new(req.clone(), response));
                    }
                    Err(err) => {
                        return Ok(ServiceResponse::from_err(err, req));
                    }
                    Ok(_) => {
                        // Do nothing
                    }
                }
            }

            let res = match Args::from_request(&req, &mut payload).await {
                Err(err) => HttpResponse::from_error(err),

                Ok(data) => handler
                    .call(data)
                    .await
                    .respond_to(&req)
                    .map_into_boxed_body(),
            };

            Ok(ServiceResponse::new(req, res))
        })
    }
}
