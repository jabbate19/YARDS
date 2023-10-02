use crate::auth::{CSHAuth, User};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, ReqData},
    HttpResponse, Responder,
};
use libyards::models::{AppState, Group};
use sqlx::{query, query_as};

#[utoipa::path(
    context_path = "/api/admin",
    responses(
        (status = 200, description = "List all IP Ranges", body = [String]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/group", wrap = "CSHAuth::enabled()")]
pub async fn get_groups(state: Data<AppState>, user: Option<ReqData<User>>) -> impl Responder {
    match query_as!(Group, "SELECT * FROM \"group\"").fetch_all(&state.db).await {
        Ok(groups) => HttpResponse::Ok().json(groups),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}