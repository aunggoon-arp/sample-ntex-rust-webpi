use ntex::web::{self, HttpRequest, HttpResponse, Responder};
use ntex_multipart::Multipart;

use crate::service::file::FileService;

pub fn file_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_file);
    cfg.service(upload_file);
}

#[web::post("/file")]
async fn hello_file(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello file controller!")
}

#[web::post("/file/upload")]
async fn upload_file(form: Multipart) -> HttpResponse {
    FileService::save_file(form).await.unwrap()
}