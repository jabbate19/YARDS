use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, ReqData},
    HttpResponse, Responder,
};
use libyards::models::{AppState, DHCPRange};
use sqlx::{query, query_as};
use crate::auth::{CSHAuth, User};

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 200, description = "List all dhcp ranges for given IP range", body = [DHCPRange]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/iprange/{iprangeid}/dhcp", wrap = "CSHAuth::admin_only()")]
pub async fn get_ip_range_dhcp(state: Data<AppState>, user: Option<ReqData<User>>, path: Path<(i32,)>) -> impl Responder {
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
#[post("/iprange/{iprangeid}/dhcp", wrap = "CSHAuth::admin_only()")]
pub async fn add_ip_range_dhcp(
    state: Data<AppState>, user: Option<ReqData<User>>,
    path: Path<(i32,)>,
    dhcp: Json<DHCPRange>,
) -> impl Responder {
    let (_iprangeid,) = path.into_inner();
    match query_as!(
        DHCPRange,
        "INSERT INTO dhcprange(iprangeid, name, dhcpstart, dhcpend, jail) VALUES ($1, $2, $3, $4, $5) RETURNING id, iprangeid, name, dhcpstart, dhcpend, jail",
        dhcp.iprangeid,
        dhcp.name,
        dhcp.dhcpstart,
        dhcp.dhcpend,
        dhcp.jail
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
#[delete("/iprange/{iprangeid}/dhcp/{dhcpid}", wrap = "CSHAuth::admin_only()")]
pub async fn delete_ip_range_dhcp(state: Data<AppState>, user: Option<ReqData<User>>, path: Path<(i32, i32)>) -> impl Responder {
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
#[put("/iprange/{iprangeid}/dhcp/{dhcpid}", wrap = "CSHAuth::admin_only()")]
pub async fn edit_ip_range_dhcp(
    state: Data<AppState>, user: Option<ReqData<User>>,
    path: Path<(i32, i32)>,
    dhcp: Json<DHCPRange>,
) -> impl Responder {
    let (_iprangeid, dhcpid) = path.into_inner();
    match query_as!(
        DHCPRange,
        "UPDATE dhcprange SET iprangeid=$1, name=$2, dhcpstart=$3, dhcpend=$4, jail=$5 WHERE id = $6 RETURNING id, iprangeid, name, dhcpstart, dhcpend, jail",
        dhcp.iprangeid,
        dhcp.name,
        dhcp.dhcpstart,
        dhcp.dhcpend,
        dhcp.jail,
        dhcpid
    )
        .fetch_one(&state.db)
        .await {
            Ok(new_dhcp) => HttpResponse::Created().json(new_dhcp),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}
