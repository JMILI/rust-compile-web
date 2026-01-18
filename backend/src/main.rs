//! Rust 编译探索 Web 服务器
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::process::Command;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use log::info;

mod compiler;
mod api;

#[derive(Clone, Serialize, Deserialize)]
struct CompileRequest {
    code: String,
    stage: String,
    options: CompileOptions,
}

#[derive(Clone, Serialize, Deserialize)]
struct CompileOptions {
    verbose: bool,
    brief: bool,
    full_output: bool,
}

#[derive(Clone)]
struct AppState {
    // 可以存储一些共享状态，比如编译缓存等
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting Rust Compile Explorer Web Server...");
    info!("Make sure you are using Rust nightly toolchain");

    // 检查 Rust 版本
    match Command::new("rustc").arg("--version").output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout);
            info!("Rust version: {}", version.trim());
            if !version.contains("nightly") {
                log::warn!("Warning: Not using nightly toolchain, some features may not work");
            }
        }
        Err(e) => {
            log::error!("Failed to get Rust version: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Rust not found"));
        }
    }

    // 共享应用状态
    let app_state = web::Data::new(AppState {});

    // 启动 HTTP 服务器
    info!("Server listening on http://127.0.0.1:8080");
    info!("Open http://127.0.0.1:5173 in browser to access frontend");

    HttpServer::new(move || {
        // 配置 CORS 以允许前端访问
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .route("/health", web::get().to(health_check))
            .route("/api/compile", web::post().to(api::compile))
            .route("/api/stages", web::get().to(api::list_stages))
            .route("/api/version", web::get().to(api::rust_version))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}