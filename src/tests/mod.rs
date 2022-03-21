mod test_builder;
mod test_permission;
mod test_service;
#[cfg(test)]
mod stubs {
    use crate::permission::{Permission, PinnedFuture};
    use actix_web::dev::Payload;
    use actix_web::{web, HttpRequest};
    use std::future::{ready, Future, Ready};
    use std::pin::Pin;
    //
    pub async fn always_true(
        _req: &HttpRequest,
        _payload: &mut Payload,
    ) -> actix_web::Result<bool> {
        Ok(true)
    }

    // pub fn always_true<'l>(
    //     _req: &'l HttpRequest,
    //     _payload: &'l mut Payload,
    // ) -> Pin<Box<dyn Future<Output = actix_web::Result<bool>> + 'l>> {
    //     Box::pin(async {
    //         _req;
    //         _payload;
    //         Ok(true)
    //     })
    // }

    pub struct AlwaysTrueStruct;
    impl Permission for AlwaysTrueStruct {
        fn call<'l>(
            &self,
            _req: &'l HttpRequest,
            _payload: &'l mut Payload,
        ) -> Pin<Box<dyn Future<Output = actix_web::Result<bool>> + 'l>> {
            Box::pin(async move {
                _req;
                _payload;
                Ok(true)
            })
        }
    }
}
