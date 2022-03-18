use actix_permissions::*;
use actix_web::web::Data;
use actix_web::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MyStatus {
    pub status: Option<String>,
}

async fn dummy_permission_check(
    _req: HttpRequest,
    dummy_service: web::Data<DummyService>,
    data: web::Query<MyStatus>,
) -> actix_web::Result<bool> {
    // Unecessary complicating permission check to show what it can do.
    // You have access to request, payload, and all injected dependencies through app_data.
    Ok(dummy_service.check(data.status.clone()))
}

struct DummyService;

impl DummyService {
    pub fn check(&self, value: Option<String>) -> bool {
        value == Some("all".to_string())
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
            .service(web::scope("").route("/", check(web::get(), dummy_permission_check, index)))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
