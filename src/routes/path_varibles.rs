use axum::extract::Path;

pub async fn path_varibles(Path(id): Path<u32>) -> String {
    id.to_string()
}
