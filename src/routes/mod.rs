pub mod custom_headers;
pub mod headers;
pub mod hello_world;
pub mod mirror_route;
pub mod mirror_route_json;
pub mod path_varibles;
pub mod query_params;
pub mod shared_data;

use tower_http::cors::{Any, CorsLayer};


use axum::{
    http::Method, routing::{get, post}, Extension, Router
};

use headers::headers;
use hello_world::get_hello_world;
use mirror_route::mirror_route;
use mirror_route_json::mirror_route_json;
use path_varibles::path_varibles;
use query_params::query_params;
use shared_data::shared_data;


#[derive(Clone)]
pub struct SharedData {
    auth: String,
}

pub async fn routes() -> Router {

    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);

    let sharedData = SharedData {auth:"test key".to_owned()};

    Router::new()
        .route("/", get(get_hello_world))
        .route("/mirror_route", post(mirror_route))
        .route("/mirror_route_json", post(mirror_route_json))
        .route("/path_varibles/{id}", get(path_varibles))
        .route("/query_params", get(query_params))
        .route("/header", get(headers))
        .route("/shared_data",get(shared_data))
        .layer(Extension(sharedData))
        .layer(cors)
}
