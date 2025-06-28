use std::sync::Arc;

use axum::Extension;

use crate::routes::SharedData;



pub async fn shared_data(Extension(shared_data):Extension<SharedData>)-> String {
    shared_data.auth
}