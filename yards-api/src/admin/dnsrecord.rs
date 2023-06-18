use crate::models::{
    AppState, DNSRecord, DNSRecordType,
};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use sqlx::{query_as};

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
