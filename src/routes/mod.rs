pub mod hello_world;
pub mod mirror_route;
pub mod mirror_route_json;
pub mod path_varibles;

use axum::{
    Router,
    routing::{get, post},
};

use hello_world::get_hello_world;
use mirror_route::mirror_route;
use mirror_route_json::mirror_route_json;
use path_varibles::path_varibles;

pub async fn routes() -> Router {
    Router::new()
        .route("/", get(get_hello_world))
        .route("/mirror_route", post(mirror_route))
        .route("/mirror_route_json", post(mirror_route_json))
        .route("/path_varibles/{id}", get(path_varibles))
}
