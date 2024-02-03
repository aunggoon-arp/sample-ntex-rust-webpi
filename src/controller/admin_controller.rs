use ntex::web::{self, HttpRequest, HttpResponse, Responder};

pub fn admin_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_admin);
}

#[utoipa::path(
    get,
    path = "/api/admin",
    responses(
        (status = 200, description = "Hello admin controller!"),
        (status = 401, description = "Invalid")
    ),
    security(
        ("Token" = []),
    )
)]
#[web::get("/admin")]
async fn hello_admin(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello admin controller!")
}