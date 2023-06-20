use crate::models::{
    AppState, IPRange, IPVersion,
};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder, delete, put,
};
use sqlx::{query_as, query};

#[utoipa::path(
    context_path = "/admin",
    responses(
        (status = 200, description = "List all IP Ranges", body = [IPRange]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/iprange")]
pub async fn get_ip_range(state: Data<AppState>) -> impl Responder {
    match query_as!(IPRange, "SELECT id, name, ipversion AS \"ipversion: IPVersion\", networkid, cidr, description FROM iprange").fetch_all(&state.db).await {
        Ok(servers) => HttpResponse::Ok().json(servers),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(
    context_path = "/admin",
    responses(
        (status = 201, description = "IP Range Added"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[post("/iprange")]
pub async fn add_ip_range(state: Data<AppState>, range: Json<IPRange>) -> impl Responder {
    match query_as!(
        IPRange,
        "INSERT INTO iprange(name, ipversion, networkid, cidr, description) VALUES ($1, $2, $3, $4, $5) RETURNING id, name, ipversion as \"ipversion: _\", networkid, cidr, description",
        range.name,
        range.ipversion as IPVersion,
        range.networkid,
        range.cidr,
        range.description
    )
        .fetch_one(&state.db)
        .await {
            Ok(new_range) => HttpResponse::Created().json(new_range),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}

#[utoipa::path(
    context_path = "/admin",
    responses(
        (status = 200, description = "IP Range Deleted"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[delete("/iprange/{rangeid}")]
pub async fn delete_ip_range(state: Data<AppState>, path: Path<(i32,)>) -> impl Responder {
    let (rangeid,) = path.into_inner();
    match query!(
        "DELETE FROM iprange WHERE id = $1",
        rangeid
    )
        .execute(&state.db)
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}

#[utoipa::path(
    context_path = "/admin",
    responses(
        (status = 200, description = "IP Range Edited"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[put("/iprange/{rangeid}")]
pub async fn edit_ip_range(state: Data<AppState>, range: Json<IPRange>, path: Path<(i32,)>) -> impl Responder {
    let (rangeid,) = path.into_inner();
    match query_as!(
        IPRange,
        "UPDATE iprange SET name=$1, ipversion=$2, networkid=$3, cidr=$4, description=$5 WHERE id=$6 RETURNING id, name, ipversion as \"ipversion: _\", networkid, cidr, description",
        range.name,
        range.ipversion as IPVersion,
        range.networkid,
        range.cidr,
        range.description,
        rangeid
    )
        .fetch_one(&state.db)
        .await {
            Ok(new_range) => HttpResponse::Created().json(new_range),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}