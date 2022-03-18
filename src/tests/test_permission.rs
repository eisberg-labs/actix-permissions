#[cfg(test)]
mod tests {
    use actix_web::dev::Payload;
    use actix_web::{test, HttpRequest};
    use std::future::{ready, Ready};

    use crate::permission::*;
    use crate::tests::stubs::{always_true, AlwaysTrueStruct};

    #[actix_web::test]
    async fn test_converts_to_permission() {
        let service_req = test::TestRequest::with_uri("/").to_srv_request();
        let (req, mut payload) = service_req.into_parts();

        let result = always_true.check(&req, &mut payload).await;

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[actix_web::test]
    async fn test_always_true_struct() {
        let service_req = test::TestRequest::with_uri("/").to_srv_request();
        let (req, mut payload) = service_req.into_parts();

        let result = AlwaysTrueStruct {}.check(&req, &mut payload).await;

        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
