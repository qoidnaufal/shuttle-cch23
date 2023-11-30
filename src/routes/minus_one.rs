use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn day_minus_one() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}
