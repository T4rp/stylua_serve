use std::{env, fs};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use stylua_lib::{Config, OutputVerification};
use tokio::net::TcpListener;

const PLUGIN_DATA: &[u8] = include_bytes!("../StyluaServePlugin.rbxm");

#[derive(serde::Deserialize)]
struct FormatRequest {
    source: String,
    options: Option<Config>,
}

async fn root() -> &'static str {
    "stylua_serve 🤗"
}

async fn format_request(body: Json<FormatRequest>) -> (StatusCode, String) {
    let source = body.0.source;
    let options = body.0.options.unwrap_or_default();

    let format_result = stylua_lib::format_code(&source, options, None, OutputVerification::None);

    match format_result {
        Ok(source) => (StatusCode::OK, source),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    }
}

async fn serve() {
    let app = Router::new()
        .route("/", get(root))
        .route("/format", post(format_request));

    let listener = TcpListener::bind("127.0.0.1:55096").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}

fn start_server() {
    println!("Listening on http://localhost:55096");

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(1)
        .thread_stack_size(6 * 1024 * 1024)
        .build()
        .unwrap()
        .block_on(async { serve().await })
}

fn install_plugin() {
    let studio = roblox_install::RobloxStudio::locate().expect("Couldn't find Roblox Studio");
    let plugin_path = studio.plugins_path().to_path_buf().join("StyluaServePlugin.rbxm");

    fs::write(&plugin_path, PLUGIN_DATA).expect("Failed to install plugin");

    println!("Installed plugin to {}", plugin_path.to_str().unwrap());
}

fn main() {
    let mut args = env::args();
    let command = args.nth(1);

    match command {
        Some(command_string) => match command_string.as_str() {
            "install-plugin" => install_plugin(),
            _ => {
                eprintln!("Unknown command \"{}\"", command_string)
            }
        },
        _ => start_server(),
    }
}
