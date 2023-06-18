use crate::models::{
    AppState, IPRange, IPVersion,
};
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use sqlx::{query_as};

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
pub async fn add_range(state: Data<AppState>, range: Json<IPRange>) -> impl Responder {
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