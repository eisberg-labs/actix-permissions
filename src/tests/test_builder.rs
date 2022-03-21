#[cfg(test)]
mod tests {
    use crate::builder::Builder;
    use crate::permission::Permission;
    use crate::permission::*;
    use crate::tests::stubs::{always_true, AlwaysTrueStruct};
    use actix_web::dev::Payload;
    use actix_web::{test, HttpRequest};
    use std::future::{ready, Ready};

    #[actix_web::test]
    async fn test_default() {
        assert!(Builder::default().permissions.is_empty());
    }

    #[actix_web::test]
    async fn test_new() {
        assert!(Builder::new().permissions.is_empty());
    }

    #[actix_web::test]
    async fn test_accepts_fn() {
        let b = Builder::new();
        // b.and(always_true);
        // assert_eq!(Builder::new().and(always_true).permissions.len(), 1);
    }

    #[actix_web::test]
    async fn test_accepts_struct() {
        let perm = AlwaysTrueStruct {};
        assert_eq!(
            Builder::new()
                .and(AlwaysTrueStruct {})
                .and(AlwaysTrueStruct {})
                .permissions
                .len(),
            2
        );
    }
}
