use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryParamsReq {
    message: String,
    id: Option<i32>,
}

pub async fn query_params(Query(query): Query<QueryParamsReq>) -> Json<QueryParamsReq> {
    match query.id {
        None => {
            return Json(QueryParamsReq {
                message: query.message,
                id: None,
            });
        }
        Some(id) => {
            return Json(QueryParamsReq {
                message: query.message,
                id: Some(id),
            });
        }
    }
}
