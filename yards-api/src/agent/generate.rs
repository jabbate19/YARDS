use crate::models::{Address, AppState, DHCPRange, DNSRecord, DNSZone, IPType, StaticAddress};
use actix_web::{
    delete, get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{
    postgres::{PgHasArrayType, PgTypeInfo},
    query, query_as,
};

#[derive(Serialize, Deserialize)]
struct DNSData {
    pub root: String,
    pub records: Option<Vec<DNSRecord>>,
}

#[utoipa::path(
    context_path = "/agent",
    responses(
        (status = 200, description = "Provide Data for Server"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/{serverid}/dns")]
async fn generate_dns_data(state: Data<AppState>, path: Path<(i32,)>) -> impl Responder {
    let (serverid,) = path.into_inner();
    match query_as!(
        DNSData,
        "SELECT dnszone.dnsroot AS \"root: String\", array_remove(ARRAY_AGG(dnsrecord.*), NULL) AS \"records: Vec<DNSRecord>\" FROM dnszone LEFT JOIN dnsrecord ON dnsrecord.zoneid = dnszone.id WHERE dnszone.serverid = $1 GROUP BY dnszone.id",
        serverid
    ).fetch_all(&state.db).await {
        Ok(zones) => HttpResponse::Ok().json(zones),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Serialize, Deserialize)]
struct DHCPData {
    pub dhcp: Option<DHCPRange>,
    pub addresses: Option<Vec<Address>>,
    pub statics: Option<Vec<Option<String>>>
}

#[utoipa::path(
    context_path = "/agent",
    responses(
        (status = 200, description = "Provide Data for Server"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/{serverid}/dhcp")]
async fn generate_dhcp_data(state: Data<AppState>, path: Path<(i32,)>) -> impl Responder {
    let (serverid,) = path.into_inner();
    match query_as!(
        DHCPData,
        "SELECT 
        ROW(dhcprange.*)::dhcprange AS \"dhcp: DHCPRange\",
        array_remove(ARRAY_AGG(address.*), NULL) AS \"addresses: _\",
        ARRAY_AGG(staticaddress.ipaddr) AS \"statics: _\"
        FROM dhcprange
        LEFT JOIN address ON dhcprange.iprangeid = address.iprangeid
        LEFT JOIN staticaddress ON staticaddress.addressid = address.id
        WHERE dhcprange.serverid = $1
        GROUP BY dhcprange.id",
        serverid
    ).fetch_all(&state.db).await {
        Ok(mut ranges) => {
        HttpResponse::Ok().json(ranges)
    },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
