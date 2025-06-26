use axum::Json;
use axum_extra::{TypedHeader, headers::UserAgent};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HeadersResponce {
    message: String,
}

pub async fn headers(TypedHeader(user_agent): TypedHeader<UserAgent>) -> Json<HeadersResponce> {
    Json(HeadersResponce {
        message: user_agent.to_string(),
    })
}
