use crate::models::{
    AppState, DHCPRange,
};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use sqlx::{query_as};

#[utoipa::path(
    context_path = "/admin",
    responses(
        (status = 200, description = "List all dhcp ranges for given IP range", body = [DHCPRange]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/iprange/{iprangeid}/dhcp")]
pub async fn get_ip_range_dhcp(state: Data<AppState>, path: Path<(i32,)>) -> impl Responder {
    let (iprangeid,) = path.into_inner();
    match query_as!(
        DHCPRange,
        "SELECT * FROM dhcprange WHERE iprangeid = $1",
        iprangeid
    )
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
        (status = 201, description = "IP Range DHCP Added"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[post("/iprange/{iprangeid}/dhcp")]
pub async fn add_ip_range_dhcp(state: Data<AppState>, dhcp: Json<DHCPRange>) -> impl Responder {
    match query_as!(
        DHCPRange,
        "INSERT INTO dhcprange(iprangeid, name, dhcpstart, dhcpend, gateway, default_dns, lease_time, serverid) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id, iprangeid, name, dhcpstart, dhcpend, gateway, default_dns, lease_Time, serverid",
        dhcp.iprangeid,
        dhcp.name,
        dhcp.dhcpstart,
        dhcp.dhcpend,
        dhcp.gateway,
        dhcp.default_dns,
        dhcp.lease_time,
        dhcp.serverid
    )
        .fetch_one(&state.db)
        .await {
            Ok(new_dhcp) => HttpResponse::Created().json(new_dhcp),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}