use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use libyards::models::{AppState, DNSZone};
use sqlx::{query, query_as};

#[utoipa::path(
    context_path = "/api/admin",
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
    context_path = "/api/admin",
    responses(
        (status = 201, description = "DNS Zone Added"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[post("/dnszone")]
pub async fn add_dns_zone(state: Data<AppState>, zone: Json<DNSZone>) -> impl Responder {
    match query_as!(
        DNSZone,
        "INSERT INTO dnszone(zonename, dnsroot, serverid, refresh, retry, expire, nxdomain, contact, server) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id, zonename, serverid, dnsroot, refresh, retry, expire, nxdomain, contact, server",
        zone.zonename,
        zone.dnsroot,
        zone.serverid,
        zone.refresh,
        zone.retry,
        zone.expire,
        zone.nxdomain,
        zone.contact,
        zone.server
    )
        .fetch_one(&state.db)
        .await {
            Ok(new_zone) => HttpResponse::Created().json(new_zone),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 200, description = "DNS Zone Deleted"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[delete("/dnszone/{zoneid}")]
pub async fn delete_dns_zone(state: Data<AppState>, path: Path<(i32,)>) -> impl Responder {
    let (zoneid,) = path.into_inner();
    match query!("DELETE FROM dnszone WHERE id = $1", zoneid)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 200, description = "DNS Zone Edited"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[put("/dnszone/{zoneid}")]
pub async fn edit_dns_zone(
    state: Data<AppState>,
    zone: Json<DNSZone>,
    path: Path<(i32,)>,
) -> impl Responder {
    let (zoneid,) = path.into_inner();
    match query_as!(
        DNSZone,
        "UPDATE dnszone SET zonename=$1, dnsroot=$2, serverid=$3, refresh=$4, retry=$5, expire=$6, nxdomain=$7, contact=$8, server=$9 WHERE id=$10 RETURNING id, zonename, serverid, dnsroot, refresh, retry, expire, nxdomain, contact, server",
        zone.zonename,
        zone.dnsroot,
        zone.serverid,
        zone.refresh,
        zone.retry,
        zone.expire,
        zone.nxdomain,
        zone.contact,
        zone.server,
        zoneid,
    )
        .fetch_one(&state.db)
        .await {
            Ok(new_zone) => HttpResponse::Created().json(new_zone),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}
