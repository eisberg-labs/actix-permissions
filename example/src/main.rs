use actix_permissions::{check, with};
use actix_web::dev::*;
use actix_web::web::Data;
use actix_web::*;
use std::future::{ready, Ready};

fn dummy_permission_check(
    req: &HttpRequest,
    _payload: &mut Payload,
) -> Ready<actix_web::Result<bool, actix_web::Error>> {
    // Unecessary complicating permission check to show what it can do.
    // You have access to request, payload, and all injected dependencies through app_data.
    let checker_service: Option<&Data<DummyService>> = req.app_data::<Data<DummyService>>();
    // TODO: do not unwrap here
    ready(Ok(checker_service.unwrap().check(req.query_string())))
}

fn another_dummy_permission_check(
    req: &HttpRequest,
    _payload: &mut Payload,
) -> Ready<actix_web::Result<bool, actix_web::Error>> {
    // Unecessary complicating permission check to show what it can do.
    // You have access to request, payload, and all injected dependencies through app_data.
    let checker_service: Option<&Data<DummyService>> = req.app_data::<Data<DummyService>>();
    // TODO: do not unwrap here
    ready(Ok(checker_service.unwrap().check(req.query_string())))
}

struct DummyService;

impl DummyService {
    pub fn check(&self, value: &str) -> bool {
        value.contains('q')
    }
}

async fn index() -> Result<String, Error> {
    Ok("Hi there!".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(DummyService))
            .service(web::scope("").route(
                "/",
                check(
                    web::get(),
                    with(dummy_permission_check).and(another_dummy_permission_check),
                    index,
                ),
            ))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
