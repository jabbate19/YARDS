use crate::auth::{CSHAuth, User};
use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, ReqData},
    HttpResponse, Responder,
};
use libyards::models::{AppState, DDNS};
use sqlx::{query, query_as};

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 200, description = "List all ddns ranges", body = [DDNS]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/ddns", wrap = "CSHAuth::admin_only()")]
pub async fn get_ddns(state: Data<AppState>) -> impl Responder {
    match query_as!(DDNS, "SELECT * FROM ddns")
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
        (status = 201, description = "DDNS Range Added"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[post("/ddns", wrap = "CSHAuth::admin_only()")]
pub async fn add_ddns(state: Data<AppState>, range: Json<DDNS>) -> impl Responder {
    match query_as!(
        DDNS,
        "INSERT INTO ddns(iprangeid, zoneid) VALUES ($1, $2) RETURNING iprangeid, zoneid",
        range.iprangeid,
        range.zoneid
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(new_range) => HttpResponse::Created().json(new_range),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 200, description = "DDNS Range Deleted"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[delete("/ddns/{iprangeid}/{zoneid}", wrap = "CSHAuth::admin_only()")]
pub async fn delete_ddns(state: Data<AppState>, path: Path<(i32, i32)>) -> impl Responder {
    let (iprangeid, zoneid) = path.into_inner();
    match query!(
        "DELETE FROM ddns WHERE iprangeid = $1 AND zoneid = $2",
        iprangeid,
        zoneid
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
