use ntex::web::{self, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use crate::{
    dto::{custom::ParamRequest, role::{CreateRoleInput, UpdateRoleInput}},
    service::role::RoleService,
    utils::jwt::jwt_verify,
    MySqlState,
};

pub fn role_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_role);
    cfg.service(get_role_by_id);
    cfg.service(get_role_all);
    cfg.service(post_create_role);
    cfg.service(put_update_role);
}

#[utoipa::path(
    get,
    path = "/api",
    responses(
        (status = 200, description = "Get role home!"),
        (status = 401, description = "Invalid")
    ),
)]
#[web::get("/role")]
async fn hello_role(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello role controller!")
}

#[utoipa::path(
    get,
    path = "/api/role/getById/{id}",
    responses(
        (status = 200, description = "Get role by id!"),
        (status = 401, description = "Invalid")
    ),
    params(
        ParamRequest,
    ),
)]
#[web::get("/role/getById/{id}")]
async fn get_role_by_id(
    _req: HttpRequest,
    path: web::types::Path<ParamRequest>,
    db_state: web::types::State<MySqlState>,
) -> HttpResponse {
    let authorize = jwt_verify(_req);
    match authorize {
        Ok(_user_claim) => {
            let param = path.into_inner();
            let result = RoleService::get_role_by_id(param.id, &db_state.db).await;
            match result {
                Ok(data) => HttpResponse::Ok().json(&data),
                Err(_err) => {
                    let json = json!({"message": _err.to_string()});
                    HttpResponse::Unauthorized().json(&json)
                }
            }
        }
        Err(_err) => {
            let json = json!({"message": "Unauthorized."});
            HttpResponse::Unauthorized().json(&json)
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/role/getAll",
    responses(
        (status = 200, description = "Get role all !"),
        (status = 401, description = "Invalid")
    ),
    params(
        ParamRequest,
    ),
)]
#[web::get("/role/getAll")]
async fn get_role_all(_req: HttpRequest, db_state: web::types::State<MySqlState>) -> HttpResponse {
    let authorize = jwt_verify(_req);
    match authorize {
        Ok(_user_claim) => {
            let result = RoleService::get_role_all(&db_state.db).await;
            match result {
                Ok(data) => HttpResponse::Ok().json(&data),
                Err(_err) => {
                    let json = json!({"message": _err.to_string()});
                    HttpResponse::Unauthorized().json(&json)
                }
            }
        }
        Err(_err) => {
            let json = json!({"message": "Unauthorized."});
            HttpResponse::Unauthorized().json(&json)
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/role/create",
    responses(
        (status = 200, description = "Create !"),
        (status = 409, description = "Invalid Request Format")
    ),
    request_body(content = String, example = json!({"name_th": "ผู้ใช้งาน", "name_en": "User", "role_code": " user"})),
)]
#[web::post("/role/create")]
async fn post_create_role(
    _req: HttpRequest,
    body: web::types::Json<CreateRoleInput>,
    db_state: web::types::State<MySqlState>,
) -> HttpResponse {
    let authorize = jwt_verify(_req);
    match authorize {
        Ok(_user_claim) => {
            let input = CreateRoleInput {
                name_th: body.name_th.clone(),
                name_en: body.name_en.clone(),
                role_code: body.role_code.clone(),
            };
            let result = RoleService::create_role(input, &db_state.db).await;
            match result {
                Ok(data) => HttpResponse::Ok().json(&data),
                Err(_err) => HttpResponse::BadRequest().body(_err.to_string()),
            }
        }
        Err(_err) => {
            let json = json!({"message": "Unauthorized."});
            HttpResponse::Unauthorized().json(&json)
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/role/update",
    responses(
        (status = 200, description = "Update role !"),
        (status = 409, description = "Invalid Request Format")
    ),
    request_body(content = String, example = json!({"id": 1, "name_th": "ผู้ใช้งาน", "name_en": "User", "role_code": " user"})),
)]
#[web::put("/role/update")]
async fn put_update_role(
    _req: HttpRequest,
    body: web::types::Json<UpdateRoleInput>,
    db_state: web::types::State<MySqlState>,
) -> HttpResponse {
    let authorize = jwt_verify(_req);
    match authorize {
        Ok(_user_claim) => {
            let input = UpdateRoleInput {
                id: body.id.clone(),
                name_th: body.name_th.clone(),
                name_en: body.name_en.clone(),
                role_code: body.role_code.clone(),
            };
            let result = RoleService::update_role(input, &db_state.db).await;
            match result {
                Ok(data) => HttpResponse::Ok().json(&data),
                Err(_err) => HttpResponse::BadRequest().body(_err.to_string()),
            }
        }
        Err(_err) => {
            let json = json!({"message": "Unauthorized."});
            HttpResponse::Unauthorized().json(&json)
        }
    }
}
