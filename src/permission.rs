use actix_web::dev::*;
use actix_web::*;
use std::future::Future;
use std::pin::Pin;

pub type PinnedFuture<O> = Pin<Box<dyn Future<Output = O>>>;
pub trait Permission {
    // type Output;
    // type Future: Future<Output = Self::Output>;
    fn call<'l>(
        &self,
        req: &'l HttpRequest,
        payload: &'l mut Payload,
    ) -> Pin<Box<dyn Future<Output = actix_web::Result<bool>> + 'l>>;
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
impl<Func, Fut> Permission for Func
where
    Func: Fn(&HttpRequest, &mut Payload) -> Fut,
    Fut: Future<Output = actix_web::Result<bool>> + 'static,
{
    #[inline]
    #[allow(non_snake_case)]
    fn call<'l>(
        &self,
        req: &'l HttpRequest,
        p: &'l mut Payload,
    ) -> Pin<Box<dyn Future<Output = actix_web::Result<bool>> + 'l>> {
        Box::pin((self)(req, p))
    }
}

// impl Clone for Box<dyn Permission> {
//     fn clone(&self) -> Box<dyn Permission> {
//         self.box_clone()
//     }
// }
