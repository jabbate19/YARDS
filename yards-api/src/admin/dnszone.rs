use crate::models::{
    AppState, DNSZone,
};
use actix_web::{
    get,
    web::{Data},
    HttpResponse, Responder,
};
use sqlx::{query_as};

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