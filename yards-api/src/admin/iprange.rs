use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, ReqData},
    HttpResponse, Responder,
};
use libyards::models::{AppState, IPRange, IPVersion};
use sqlx::{query, query_as};
use crate::auth::{CSHAuth, User};

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 200, description = "List all IP Ranges", body = [IPRange]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/iprange", wrap = "CSHAuth::admin_only()")]
pub async fn get_ip_range(state: Data<AppState>, user: Option<ReqData<User>>) -> impl Responder {
    match query_as!(IPRange, "SELECT id, name, ipversion AS \"ipversion: IPVersion\", networkid, cidr, description, vlan, gateway, default_dns, dns_domain, default_lease_time, max_lease_time, min_lease_time FROM iprange").fetch_all(&state.db).await {
        Ok(servers) => HttpResponse::Ok().json(servers),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 201, description = "IP Range Added"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[post("/iprange", wrap = "CSHAuth::admin_only()")]
pub async fn add_ip_range(state: Data<AppState>, user: Option<ReqData<User>>, range: Json<IPRange>) -> impl Responder {
    match query_as!(
        IPRange,
        "INSERT INTO iprange(name, ipversion, networkid, cidr, description, vlan, gateway, default_dns, dns_domain, default_lease_time, max_lease_time, min_lease_time) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING id, name, ipversion as \"ipversion: _\", networkid, cidr, description, vlan, gateway, default_dns, dns_domain, default_lease_time, max_lease_time, min_lease_time",
        range.name,
        range.ipversion as IPVersion,
        range.networkid,
        range.cidr,
        range.description,
        range.vlan,
        range.gateway,
        range.default_dns,
        range.dns_domain,
        range.default_lease_time,
        range.max_lease_time,
        range.min_lease_time
    )
        .fetch_one(&state.db)
        .await {
            Ok(new_range) => HttpResponse::Created().json(new_range),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 200, description = "IP Range Deleted"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[delete("/iprange/{rangeid}", wrap = "CSHAuth::admin_only()")]
pub async fn delete_ip_range(state: Data<AppState>, user: Option<ReqData<User>>, path: Path<(i32,)>) -> impl Responder {
    let (rangeid,) = path.into_inner();
    match query!("DELETE FROM iprange WHERE id = $1", rangeid)
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
        (status = 200, description = "IP Range Edited"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[put("/iprange/{rangeid}", wrap = "CSHAuth::admin_only()")]
pub async fn edit_ip_range(
    state: Data<AppState>, user: Option<ReqData<User>>,
    range: Json<IPRange>,
    path: Path<(i32,)>,
) -> impl Responder {
    let (rangeid,) = path.into_inner();
    match query_as!(
        IPRange,
        "UPDATE iprange SET name=$1, ipversion=$2, networkid=$3, cidr=$4, description=$5, vlan=$6, gateway=$7, default_dns=$8, default_lease_time=$9, max_lease_time=$10, min_lease_time=$11 WHERE id=$12 RETURNING id, name, ipversion as \"ipversion: _\", networkid, cidr, description, vlan, gateway, default_dns, dns_domain, default_lease_time, max_lease_time, min_lease_time",
        range.name,
        range.ipversion as IPVersion,
        range.networkid,
        range.cidr,
        range.description,
        range.vlan,
        range.gateway,
        range.default_dns,
        range.default_lease_time,
        range.max_lease_time,
        range.min_lease_time,
        rangeid
    )
        .fetch_one(&state.db)
        .await {
            Ok(new_range) => HttpResponse::Created().json(new_range),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}
