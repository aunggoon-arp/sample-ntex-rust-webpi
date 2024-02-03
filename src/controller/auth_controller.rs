use ntex::web::{self, HttpRequest, HttpResponse, Responder};

pub fn auth_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_admin);
}

#[web::get("/auth")]
async fn hello_admin(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello auth controller!")
}