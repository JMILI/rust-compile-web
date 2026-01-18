//! API 路由处理
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::compiler;

#[derive(Debug, Deserialize)]
pub struct CompileRequest {
    pub code: String,
    pub stage: String,
    pub options: Option<CompileOptionsRequest>,
}

#[derive(Debug, Deserialize)]
pub struct CompileOptionsRequest {
    pub verbose: Option<bool>,
    pub brief: Option<bool>,
    pub full_output: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct CompileResponse {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub stage: String,
    pub stage_name: String,
    pub stage_description: String,
    pub lines: usize,
    pub truncated: bool,
    pub rust_version: String,
}

#[derive(Debug, Serialize)]
pub struct StageInfo {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct StagesResponse {
    pub stages: Vec<StageInfo>,
}

#[derive(Debug, Serialize)]
pub struct VersionResponse {
    pub version: String,
    pub is_nightly: bool,
}

/// 处理编译请求
pub async fn compile(request: web::Json<CompileRequest>) -> impl Responder {
    // 解析编译阶段
    let stage = match compiler::CompileStage::from_str(&request.stage) {
        Some(s) => s,
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Unknown compile stage: {}", request.stage)
            }));
        }
    };

    // 设置编译选项
    let options = request.options.as_ref().map_or_else(compiler::CompileOptions::default, |opt| {
        compiler::CompileOptions {
            verbose: opt.verbose.unwrap_or(false),
            brief: opt.brief.unwrap_or(false),
            full_output: opt.full_output.unwrap_or(false),
        }
    });

    // 运行编译探索
    match compiler::explore_compile(&request.code, stage, &options) {
        Ok(result) => {
            let rust_version = compiler::get_rust_version().unwrap_or_else(|_| "Unknown".to_string());

            HttpResponse::Ok().json(CompileResponse {
                success: result.success,
                output: result.output,
                error: result.error,
                stage: request.stage.clone(),
                stage_name: stage.name().to_string(),
                stage_description: stage.description().to_string(),
                lines: result.lines,
                truncated: result.truncated,
                rust_version,
            })
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Internal server error: {}", err)
            }))
        }
    }
}

/// 获取可用的编译阶段列表
pub async fn list_stages() -> impl Responder {
    let stages = compiler::get_available_stages();
    let stage_infos: Vec<StageInfo> = stages.iter().map(|stage| {
        StageInfo {
            id: match stage {
                compiler::CompileStage::Ast => "ast",
                compiler::CompileStage::Hir => "hir",
                compiler::CompileStage::Mir => "mir",
                compiler::CompileStage::MirCfg => "mir-cfg",
                compiler::CompileStage::Expanded => "expanded",
                compiler::CompileStage::Llvm => "llvm",
                compiler::CompileStage::Asm => "asm",
            }.to_string(),
            name: stage.name().to_string(),
            description: stage.description().to_string(),
        }
    }).collect();

    HttpResponse::Ok().json(StagesResponse { stages: stage_infos })
}

/// 获取 Rust 版本信息
pub async fn rust_version() -> impl Responder {
    match compiler::get_rust_version() {
        Ok(version) => {
            let is_nightly = version.contains("nightly");
            HttpResponse::Ok().json(VersionResponse {
                version,
                is_nightly,
            })
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to get Rust version: {}", err)
            }))
        }
    }
}