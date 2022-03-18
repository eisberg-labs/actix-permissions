mod test_builder;
mod test_permission;
// mod test_service;
#[cfg(test)]
mod stubs {
    use crate::permission::{Permission, PinnedFuture};
    use actix_web::dev::Payload;
    use actix_web::HttpRequest;
    use std::future::{ready, Ready};

    pub async fn always_true(
        _req: &HttpRequest,
        _payload: &mut Payload,
    ) -> actix_web::Result<bool> {
        Ok(true)
    }

    pub struct AlwaysTrueStruct;
    impl<'r> Permission<'r> for AlwaysTrueStruct {
        fn check(
            &self,
            _req: &'r HttpRequest,
            _payload: &'r mut Payload,
        ) -> PinnedFuture<'r, actix_web::Result<bool>> {
            Box::pin(async { Ok(true) })
        }
    }
}
