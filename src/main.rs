use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use stylua_lib::Config;

async fn root() -> &'static str {
    "hello world"
}

#[derive(serde::Deserialize)]
struct FormatRequest {
    source: String,
    options: Option<Config>,
}

async fn format_request(body: Json<FormatRequest>) -> (StatusCode, String) {
    let source = body.0.source;
    let options = body.0.options.unwrap_or_default();

    let format_result =
        stylua_lib::format_code(&source, options, None, stylua_lib::OutputVerification::None);

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
