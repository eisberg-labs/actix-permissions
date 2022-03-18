use actix_web::dev::*;
use actix_web::*;
use std::future::Future;
use std::pin::Pin;

pub type PinnedFuture<'r, O> = Pin<Box<dyn Future<Output = O> + 'r>>;
pub trait Permission<'r> {
    // type Output;
    // type Future: Future<Output = Self::Output>;
    // type Future: Future<Output = actix_web::Result<bool>> + 'r;
    fn check(
        &self,
        req: &'r HttpRequest,
        payload: &'r mut Payload,
    ) -> PinnedFuture<'r, actix_web::Result<bool>>;
}

// /// A trait needed for cloning a boxed trait object
// pub trait CloneablePermission {
//     fn box_clone(&self) -> Box<dyn Permission>;
// }
//
// impl<T> CloneablePermission for T
// where
//     T: Permission + Clone + 'static,
// {
//     fn box_clone(&self) -> Box<dyn Permission> {
//         Box::new(self.clone())
//     }
// }

/// Magic that allows function as argument, instead of a Permission trait
impl<'r, Func, Fut> Permission<'r> for Func
where
    Func: Fn(&'r HttpRequest, &'r mut Payload) -> Fut,
    Fut: Future<Output = actix_web::Result<bool>>,
    Fut: 'r,
{
    // type Future = Fut;

    #[inline]
    #[allow(non_snake_case)]
    fn check(
        &self,
        req: &'r HttpRequest,
        p: &'r mut Payload,
    ) -> PinnedFuture<'r, actix_web::Result<bool>> {
        Box::pin((self)(req, p))
    }
}

// impl Clone for Box<dyn Permission> {
//     fn clone(&self) -> Box<dyn Permission> {
//         self.box_clone()
//     }
// }
