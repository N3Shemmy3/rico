use axum::Json;
use axum::response::IntoResponse;

pub async fn get() -> impl IntoResponse {
    Json("Rico-chan: moshi moshi").into_response()
}
