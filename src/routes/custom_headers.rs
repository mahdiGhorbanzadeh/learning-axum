use axum::http::HeaderMap;

pub async fn custom_headers(headers: HeaderMap) -> String {
    let x_message = headers
        .get("x-message")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    return x_message;
}
