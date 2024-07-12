use axum::{
    http::StatusCode, routing::{get, post}, Json, Router
};
use serde_json::Value;
use stylua_lib::Config;

async fn root() -> &'static str {
    "hello world"
}

async fn format_request(body: Json<Value>) -> (StatusCode, String) {
    let source = match body.get("source").and_then(|val| val.as_str()) {
        Some(source) => source,
        None => return (StatusCode::BAD_REQUEST, "Missing source".to_owned()),
    };

    let config = Config {
        ..Default::default()
    };

    let format_result =
        stylua_lib::format_code(source, config, None, stylua_lib::OutputVerification::None);

    match format_result {
        Ok(source) => (StatusCode::OK, source),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/format", post(format_request));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:55096")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap()
}
