use crate::models::{
    AppState, Server,
};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder, delete,
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
pub async fn add_server(state: Data<AppState>, server: Json<Server>) -> impl Responder {
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
        (status = 200, description = "Server Deleted"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[delete("/server/{serverid}")]
pub async fn delete_server(state: Data<AppState>, path: Path<(i32,)>) -> impl Responder {
    let (serverid,) = path.into_inner();
    match query!(
        "DELETE FROM server WHERE id = $1",
        serverid
    )
        .execute(&state.db)
        .await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
}
