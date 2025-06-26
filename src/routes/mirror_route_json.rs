use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MirrorRouteReq {
    message: String,
}

#[derive(Serialize)]
pub struct MirrorRouteRes {
    message: String,
    serverMessage: String,
}

pub async fn mirror_route_json(Json(body): Json<MirrorRouteReq>) -> Json<MirrorRouteRes> {
    let message = body.message;

    Json(MirrorRouteRes {
        message: message.clone(),
        serverMessage: String::from(message + " Server Response "),
    })
}
