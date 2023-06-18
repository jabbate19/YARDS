use crate::models::{
    AppState,
    DDNS,
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