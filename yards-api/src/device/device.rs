use crate::auth::{CSHAuth, User};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, ReqData},
    HttpResponse, Responder,
};
use libyards::models::{
    Address, AppState, DNSRecord, Device, Interface, MXRecord, SRVRecord, StaticAddress, IPType,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};

#[utoipa::path(
    context_path = "/api/device",
    responses(
        (status = 200, description = "List all IP Ranges", body = [IPRange]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/", wrap = "CSHAuth::enabled()")]
pub async fn get_devices(state: Data<AppState>, user: Option<ReqData<User>>) -> impl Responder {
    match query_as!(
        Device,
        "SELECT id, name, owner, comments FROM device WHERE owner = $1",
        user.unwrap().preferred_username
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(devices) => HttpResponse::Ok().json(devices),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize, Serialize)]
struct DeviceData {
    interface: Option<Interface>,
    addresses: Option<Vec<Address>>,
    static_addresses: Option<Vec<StaticAddress>>,
    dns_records: Option<Vec<DNSRecord>>,
    mx_records: Option<Vec<MXRecord>>,
    srv_records: Option<Vec<SRVRecord>>,
}

#[derive(Deserialize, Serialize)]
struct SearchData {
    dev_id: Option<i32>,
    name: Option<String>,
    macaddr: Option<String>,
    owner: Option<String>,
    iptype: Option<IPType>,
    ipaddr: Option<String>,
    iprangeid: Option<i32>,
    iprange: Option<String>,
}

#[utoipa::path(
    context_path = "/api/device",
    responses(
        (status = 200, description = "List all IP Ranges", body = [IPRange]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/search", wrap = "CSHAuth::enabled()")]
pub async fn search_data(
    state: Data<AppState>,
    user: Option<ReqData<User>>,
) -> impl Responder {
    match query_as!(SearchData, "
    SELECT device.id AS \"dev_id: _\", device.name AS \"name: _\", interface.macaddr AS \"macaddr: _\", device.owner AS \"owner: _\", address.iptype AS \"iptype: _\", staticaddress.ipaddr AS \"ipaddr: _\", iprange.id AS \"iprangeid: _\", iprange.name AS \"iprange: _\"
    FROM device
    JOIN interface ON interface.deviceid = device.id
    JOIN address ON address.interfaceid = interface.id
    LEFT JOIN staticaddress ON staticaddress.addressid = address.id
    JOIN iprange ON iprange.id = address.iprangeid;")
        .fetch_all(&state.db)
        .await
    {
        Ok(devices) => HttpResponse::Ok().json(devices),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}


#[utoipa::path(
    context_path = "/api/device",
    responses(
        (status = 200, description = "List all IP Ranges", body = [IPRange]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/{id}", wrap = "CSHAuth::enabled()")]
pub async fn get_device_info(
    state: Data<AppState>,
    path: Path<(i32,)>,
    user: Option<ReqData<User>>,
) -> impl Responder {
    let (deviceid,) = path.into_inner();
    match query_as!(DeviceData, "SELECT ROW(interface.*)::interface AS \"interface: _\", ARRAY_AGG(DISTINCT address.*) AS \"addresses: _\", ARRAY_REMOVE(ARRAY_AGG(staticaddress.*), NULL) AS \"static_addresses: _\", ARRAY_REMOVE(ARRAY_AGG(dnsrecord.*), NULL) AS \"dns_records: _\", ARRAY_REMOVE(ARRAY_AGG(mxrecord.*), NULL) AS \"mx_records: _\", ARRAY_REMOVE(ARRAY_AGG(srvrecord.*), NULL) AS \"srv_records: _\" FROM public.interface JOIN public.address ON address.interfaceid = interface.id LEFT JOIN public.staticaddress ON staticaddress.addressid = address.id LEFT JOIN public.dnsrecord ON dnsrecord.addrid = address.id LEFT JOIN public.mxrecord ON dnsrecord.id = mxrecord.id LEFT JOIN public.srvrecord ON srvrecord.id = dnsrecord.id WHERE deviceid = $1 GROUP BY interface.id;", deviceid)
        .fetch_all(&state.db)
        .await
    {
        Ok(devices) => HttpResponse::Ok().json(devices),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}
