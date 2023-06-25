use libyards::models::{AppState, DHCPRange};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use sqlx::{query, query_as};

#[utoipa::path(
    context_path = "/api/admin",
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
    context_path = "/api/admin",
    responses(
        (status = 201, description = "IP Range DHCP Added"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[post("/iprange/{iprangeid}/dhcp")]
pub async fn add_ip_range_dhcp(
    state: Data<AppState>,
    path: Path<(i32,)>,
    dhcp: Json<DHCPRange>,
) -> impl Responder {
    let (_iprangeid,) = path.into_inner();
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

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 200, description = "DHCP Range Deleted"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[delete("/iprange/{iprangeid}/dhcp/{dhcpid}")]
pub async fn delete_ip_range_dhcp(state: Data<AppState>, path: Path<(i32, i32)>) -> impl Responder {
    let (_iprangeid, dhcpid) = path.into_inner();
    match query!("DELETE FROM dhcprange WHERE id = $1", dhcpid)
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
        (status = 200, description = "IP Range DHCP Edited"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[put("/iprange/{iprangeid}/dhcp/{dhcpid}")]
pub async fn edit_ip_range_dhcp(
    state: Data<AppState>,
    path: Path<(i32, i32)>,
    dhcp: Json<DHCPRange>,
) -> impl Responder {
    let (_iprangeid, dhcpid) = path.into_inner();
    match query_as!(
        DHCPRange,
        "UPDATE dhcprange SET iprangeid=$1, name=$2, dhcpstart=$3, dhcpend=$4, gateway=$5, default_dns=$6, lease_time=$7, serverid=$8 WHERE id = $9 RETURNING id, iprangeid, name, dhcpstart, dhcpend, gateway, default_dns, lease_Time, serverid",
        dhcp.iprangeid,
        dhcp.name,
        dhcp.dhcpstart,
        dhcp.dhcpend,
        dhcp.gateway,
        dhcp.default_dns,
        dhcp.lease_time,
        dhcp.serverid,
        dhcpid
    )
        .fetch_one(&state.db)
        .await {
            Ok(new_dhcp) => HttpResponse::Created().json(new_dhcp),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}
