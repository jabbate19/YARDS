use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, ReqData},
    HttpResponse, Responder,
};
use libyards::models::{AppState, Server};
use passwords::PasswordGenerator;
use serde_json::json;
use sqlx::{query, query_as};
use crate::auth::{CSHAuth, User};

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 200, description = "List all servers", body = [Server]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/server", wrap = "CSHAuth::admin_only()")]
pub async fn get_servers(state: Data<AppState>, user: Option<ReqData<User>>) -> impl Responder {
    match query_as!(Server, "SELECT * FROM server")
        .fetch_all(&state.db)
        .await
    {
        Ok(servers) => HttpResponse::Ok().json(servers),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 201, description = "Server Added, Reterns JSON with new token"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[post("/server", wrap = "CSHAuth::admin_only()")]
pub async fn add_server(state: Data<AppState>, user: Option<ReqData<User>>, server: Json<Server>) -> impl Responder {
    let passgen = PasswordGenerator::new()
        .length(16)
        .uppercase_letters(true)
        .symbols(true)
        .strict(true);
    let new_token = passgen.generate_one().unwrap();
    match query!(
        "INSERT INTO server(name, tokenhash) VALUES ($1, encode(digest($2, 'sha512'), 'hex'))",
        server.name,
        new_token
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => HttpResponse::Created().json(json!({ "token": new_token })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 200, description = "Server Deleted"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[delete("/server/{serverid}", wrap = "CSHAuth::admin_only()")]
pub async fn delete_server(state: Data<AppState>, user: Option<ReqData<User>>, path: Path<(i32,)>) -> impl Responder {
    let (serverid,) = path.into_inner();
    match query!("DELETE FROM server WHERE id = $1", serverid)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
