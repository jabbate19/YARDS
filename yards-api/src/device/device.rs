use crate::auth::{CSHAuth, User};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, ReqData},
    HttpResponse, Responder,
};
use libyards::models::{AppState, Device, Interface, Address, StaticAddress};
use sqlx::{query, query_as};
use serde::{Serialize, Deserialize};

#[utoipa::path(
    context_path = "/api/device",
    responses(
        (status = 200, description = "List all IP Ranges", body = [IPRange]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/", wrap = "CSHAuth::enabled()")]
pub async fn get_devices(state: Data<AppState>, user: Option<ReqData<User>>) -> impl Responder {
    match query_as!(Device, "SELECT id, name, owner, comments FROM device WHERE owner = $1", user.unwrap().preferred_username)
        .fetch_all(&state.db)
        .await
    {
        Ok(devices) => HttpResponse::Ok().json(devices),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize, Serialize)]
struct DeviceData {
    interfaces: Option<Vec<Interface>>,
    addresses: Option<Vec<Address>>,
    static_addresses: Option<Vec<Option<StaticAddress>>>,
}

#[utoipa::path(
    context_path = "/api/device",
    responses(
        (status = 200, description = "List all IP Ranges", body = [IPRange]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/{id}", wrap = "CSHAuth::enabled()")]
pub async fn get_device_info(state: Data<AppState>, path: Path<(i32,)>, user: Option<ReqData<User>>) -> impl Responder {
    let (deviceid,) = path.into_inner();
    match query_as!(DeviceData, "SELECT ARRAY_AGG(interface.*) AS \"interfaces: _\", ARRAY_AGG(address.*) AS \"addresses: _\", ARRAY_AGG(staticaddress.*) AS \"static_addresses: _\" FROM public.interface JOIN public.address ON address.interfaceid = interface.id LEFT JOIN public.staticaddress ON staticaddress.addressid = address.id WHERE deviceid = $1", deviceid)
        .fetch_one(&state.db)
        .await
    {
        Ok(devices) => HttpResponse::Ok().json(devices),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
