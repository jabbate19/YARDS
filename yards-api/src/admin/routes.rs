use crate::models::{
    AppState, DHCPRange, DNSRecord, DNSRecordType, DNSZone, IPRange, IPVersion, Server,
    DDNS,
};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use base64::{engine::general_purpose, Engine as _};
use passwords::PasswordGenerator;
use serde_json::json;
use sha2::{Digest, Sha512};
use sqlx::{query, query_as};

#[utoipa::path(
    context_path = "/admin",
    responses(
        (status = 200, description = "List all servers", body = [Server]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/server")]
pub async fn get_servers(state: Data<AppState>) -> impl Responder {
    match query_as!(Server, "SELECT * FROM server")
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
        (status = 201, description = "Server Added, Reterns JSON with new token"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[post("/server")]
pub async fn register_server(state: Data<AppState>, server: Json<Server>) -> impl Responder {
    let passgen = PasswordGenerator::new()
        .length(16)
        .uppercase_letters(true)
        .symbols(true)
        .strict(true);
    let new_token = passgen.generate_one().unwrap();
    let mut hasher = Sha512::new();
    hasher.update(&new_token);
    let result = hasher.finalize();
    match query!(
        "INSERT INTO server(name, tokenhash) VALUES ($1, $2)",
        server.name,
        general_purpose::STANDARD_NO_PAD.encode(result)
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => HttpResponse::Created().json(json!({ "token": new_token })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

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
        (status = 200, description = "List all ddns ranges", body = [DDNS]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/ddns")]
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
        (status = 200, description = "List all DNS Records for given Zone", body = [DNSRecord]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/dnszone/{zoneid}/records")]
pub async fn get_dns_zone_records(state: Data<AppState>, path: Path<(i32,)>) -> impl Responder {
    let (zoneid,) = path.into_inner();
    match query_as!(DNSRecord, "SELECT id, zoneid, key, recordtype AS \"recordtype: DNSRecordType\", ttl, value FROM dnsrecord WHERE zoneid = $1", zoneid)
        .fetch_all(&state.db)
        .await
    {
        Ok(servers) => HttpResponse::Ok().json(servers),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
