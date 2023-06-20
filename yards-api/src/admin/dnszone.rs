use crate::models::{
    AppState, DNSZone,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
    HttpResponse, Responder, post, delete, put,
};
use sqlx::{query_as, query};

#[utoipa::path(
    context_path = "/admin",
    responses(
        (status = 200, description = "List all dns zones", body = [DNSZone]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/dnszone")]
pub async fn get_dns_zones(state: Data<AppState>) -> impl Responder {
    match query_as!(DNSZone, "SELECT * FROM dnszone")
        .fetch_all(&state.db)
        .await
    {
        Ok(servers) => HttpResponse::Ok().json(servers),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(
    context_path = "/admin",
    responses(
        (status = 201, description = "DNS Zone Added"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[post("/dnszone")]
pub async fn add_dns_zone(state: Data<AppState>, zone: Json<DNSZone>) -> impl Responder {
    match query_as!(
        DNSZone,
        "INSERT INTO dnszone(zonename, dnsroot, serverid) VALUES ($1, $2, $3) RETURNING id, zonename, dnsroot, serverid",
        zone.zonename,
        zone.dnsroot,
        zone.serverid
    )
        .fetch_one(&state.db)
        .await {
            Ok(new_zone) => HttpResponse::Created().json(new_zone),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}

#[utoipa::path(
    context_path = "/admin",
    responses(
        (status = 200, description = "DNS Zone Deleted"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[delete("/dnszone/{zoneid}")]
pub async fn delete_dns_zone(state: Data<AppState>, path: Path<(i32,)>) -> impl Responder {
    let (zoneid,) = path.into_inner();
    match query!(
        "DELETE FROM dnszone WHERE id = $1",
        zoneid
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
        (status = 200, description = "DNS Zone Edited"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[put("/dnszone/{zoneid}")]
pub async fn edit_dns_zone(state: Data<AppState>, zone: Json<DNSZone>, path: Path<(i32,)>) -> impl Responder {
    let (zoneid,) = path.into_inner();
    match query_as!(
        DNSZone,
        "UPDATE dnszone SET zonename=$1, dnsroot=$2, serverid=$3 WHERE id=$4 RETURNING id, zonename, dnsroot, serverid",
        zone.zonename,
        zone.dnsroot,
        zone.serverid,
        zoneid
    )
        .fetch_one(&state.db)
        .await {
            Ok(new_zone) => HttpResponse::Created().json(new_zone),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}