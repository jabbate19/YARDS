use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use libyards::models::{AppState, DNSRecord, DNSRecordType};
use sqlx::{query, query_as};

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 200, description = "List all DNS Records for given Zone", body = [DNSRecord]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/dnszone/{zoneid}/record")]
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

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 201, description = "DNS Record Added"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[post("/dnszone/{zoneid}/record")]
pub async fn add_dns_zone_record(
    state: Data<AppState>,
    path: Path<(i32,)>,
    record: Json<DNSRecord>,
) -> impl Responder {
    let (_zoneid,) = path.into_inner();
    match query_as!(
        DNSRecord,
        "INSERT INTO dnsrecord(zoneid, key, ttl, value, recordtype) VALUES ($1, $2, $3, $4, $5) RETURNING id, zoneid, key, ttl, value, recordtype as \"recordtype: _\"",
        record.zoneid,
        record.key,
        record.ttl,
        record.value,
        record.recordtype as DNSRecordType
    )
        .fetch_one(&state.db)
        .await {
            Ok(new_record) => HttpResponse::Created().json(new_record),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 200, description = "DNS Record Deleted"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[delete("/dnszone/{zoneid}/record/{recordid}")]
pub async fn delete_dns_zone_record(
    state: Data<AppState>,
    path: Path<(i32, i32)>,
) -> impl Responder {
    let (_zoneid, recordid) = path.into_inner();
    match query!("DELETE FROM dnsrecord WHERE id = $1", recordid)
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
        (status = 201, description = "DNS Record Edited"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[put("/dnszone/{zoneid}/record/{recordid}")]
pub async fn edit_dns_zone_record(
    state: Data<AppState>,
    path: Path<(i32, i32)>,
    record: Json<DNSRecord>,
) -> impl Responder {
    let (_zoneid, recordid) = path.into_inner();
    match query_as!(
        DNSRecord,
        "UPDATE dnsrecord SET zoneid=$1, key=$2, ttl=$3, value=$4, recordtype=$5 WHERE id=$6 RETURNING id, zoneid, key, ttl, value, recordtype as \"recordtype: _\"",
        record.zoneid,
        record.key,
        record.ttl,
        record.value,
        record.recordtype as DNSRecordType,
        recordid
    )
        .fetch_one(&state.db)
        .await {
            Ok(new_record) => HttpResponse::Created().json(new_record),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}
