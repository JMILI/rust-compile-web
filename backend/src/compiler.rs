//! 编译探索核心模块 - 专门为 Web API 设计
use std::process::Command;
use tempfile::tempdir;
use rand::Rng;
use std::fs;
use anyhow::{Result, Context, anyhow};

/// 编译阶段类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompileStage {
    Ast,
    Hir,
    Mir,
    MirCfg,
    Expanded,
    Llvm,
    Asm,
}

impl CompileStage {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ast" => Some(CompileStage::Ast),
            "hir" => Some(CompileStage::Hir),
            "mir" => Some(CompileStage::Mir),
            "mir-cfg" | "mir_cfg" | "cfg" => Some(CompileStage::MirCfg),
            "expanded" | "expand" => Some(CompileStage::Expanded),
            "llvm" | "llvm-ir" => Some(CompileStage::Llvm),
            "asm" | "assembly" => Some(CompileStage::Asm),
            _ => None,
        }
    }

    pub fn get_rustc_args(&self) -> Vec<&'static str> {
        match self {
            CompileStage::Ast => vec!["-Z", "unpretty=ast-tree"],
            CompileStage::Hir => vec!["-Z", "unpretty=hir"],
            CompileStage::Mir => vec!["-Z", "unpretty=mir"],
            CompileStage::MirCfg => vec!["-Z", "unpretty=mir-cfg"],
            CompileStage::Expanded => vec!["-Z", "unpretty=expanded"],
            CompileStage::Llvm => vec!["--emit=llvm-ir"],
            CompileStage::Asm => vec!["--emit=asm", "-C", "llvm-args=-x86-asm-syntax=intel"],
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            CompileStage::Ast => "抽象语法树 (AST)",
            CompileStage::Hir => "高级中间表示 (HIR)",
            CompileStage::Mir => "中级中间表示 (MIR)",
            CompileStage::MirCfg => "MIR 控制流图",
            CompileStage::Expanded => "宏展开",
            CompileStage::Llvm => "LLVM 中间表示",
            CompileStage::Asm => "汇编代码",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            CompileStage::Ast => "展示代码的语法结构，包括函数定义、变量声明、表达式结构等。",
            CompileStage::Hir => "经过语义分析的中间表示，包含完整的类型信息、名称解析、泛型实例化和生命周期信息。",
            CompileStage::Mir => "Rust 特有的中间表示，用于所有权检查和优化，包括基本块、控制流图、所有权和借用操作。",
            CompileStage::MirCfg => "以文本形式展示 MIR 的控制流图，便于理解程序流程。",
            CompileStage::Expanded => "显示所有宏扩展后的完整代码，包括标准库中的宏。",
            CompileStage::Llvm => "Rust 使用 LLVM 作为后端，这是生成的 LLVM 中间表示，包括函数定义和调用、内存操作。",
            CompileStage::Asm => "最终的机器码，与目标平台相关，包括 x86/x86_64 汇编、系统调用、函数调用约定。",
        }
    }
}

/// 编译选项
#[derive(Debug, Clone)]
pub struct CompileOptions {
    pub verbose: bool,
    pub brief: bool,
    pub full_output: bool,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            verbose: false,
            brief: false,
            full_output: false,
        }
    }
}

/// 编译结果
#[derive(Debug, Clone)]
pub struct CompileResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub stage: CompileStage,
    pub lines: usize,
    pub truncated: bool,
}

/// 运行编译探索
pub fn explore_compile(
    source: &str,
    stage: CompileStage,
    options: &CompileOptions,
) -> Result<CompileResult> {
    let args = stage.get_rustc_args();

    // 创建临时目录
    let temp_dir = tempdir()?;
    let rand_num: u32 = rand::thread_rng().gen();
    let temp_rs_path = temp_dir.path().join(format!("temp_{}.rs", rand_num));

    // 写入源代码
    fs::write(&temp_rs_path, source)
        .with_context(|| format!("Failed to write temp file: {:?}", temp_rs_path))?;

    // 特殊处理 LLVM IR 和汇编代码的输出
    if stage == CompileStage::Llvm || stage == CompileStage::Asm {
        let output_ext = if stage == CompileStage::Llvm { "ll" } else { "s" };
        let output_path = temp_dir.path().join(format!("output.{}", output_ext));

        let mut cmd = Command::new("rustc");
        cmd.arg(&temp_rs_path)
            .arg("--crate-name")
            .arg(format!("temp_{}", rand_num))
            .arg("--color")
            .arg("never");

        // 添加特定参数
        for arg in args.iter() {
            cmd.arg(arg);
        }

        cmd.arg("-o").arg(&output_path);

        let status = cmd.status()?;

        if status.success() && output_path.exists() {
            let output = fs::read_to_string(&output_path)?;
            let (processed_output, truncated) = process_output(&output, options);

            return Ok(CompileResult {
                success: true,
                output: processed_output,
                error: None,
                stage,
                lines: output.lines().count(),
                truncated,
            });
        } else {
            return Ok(CompileResult {
                success: false,
                output: String::new(),
                error: Some(format!("Failed to generate {} output", stage.name())),
                stage,
                lines: 0,
                truncated: false,
            });
        }
    }

    // 处理其他编译阶段（AST、HIR、MIR 等）
    let mut cmd = Command::new("rustc");
    cmd.arg(&temp_rs_path)
        .arg("--crate-name")
        .arg(format!("temp_{}", rand_num))
        .arg("--color")
        .arg("never");

    for arg in args.iter() {
        cmd.arg(arg);
    }

    let output = cmd.output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let (processed_output, truncated) = process_output(&stdout, options);

        Ok(CompileResult {
            success: true,
            output: processed_output,
            error: None,
            stage,
            lines: stdout.lines().count(),
            truncated,
        })
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        Ok(CompileResult {
            success: false,
            output: String::new(),
            error: Some(format!("Rustc error: {}", stderr)),
            stage,
            lines: 0,
            truncated: false,
        })
    }
}

/// 处理输出，根据选项进行截断
fn process_output(output: &str, options: &CompileOptions) -> (String, bool) {
    if options.full_output {
        return (output.to_string(), false);
    }

    let lines: Vec<&str> = output.lines().collect();
    let max_lines = if options.brief { 100 } else { 1000 };

    if lines.len() > max_lines {
        let truncated: Vec<&str> = lines[..max_lines].to_vec();
        let mut result = truncated.join("\n");
        result.push_str(&format!("\n... (共{}行，显示前{}行)", lines.len(), max_lines));
        (result, true)
    } else {
        (output.to_string(), false)
    }
}

/// 获取可用的编译阶段
pub fn get_available_stages() -> Vec<CompileStage> {
    vec![
        CompileStage::Ast,
        CompileStage::Hir,
        CompileStage::Mir,
        CompileStage::MirCfg,
        CompileStage::Expanded,
        CompileStage::Llvm,
        CompileStage::Asm,
    ]
}
/// 获取 JSON 格式的 AST
pub fn get_ast_json(source: &str) -> Result<serde_json::Value> {
    let temp_dir = tempdir()?;
    let temp_rs_path = temp_dir.path().join("temp_ast.rs");

    fs::write(&temp_rs_path, source)?;

    // 使用 rustc 输出 JSON 格式的 AST
    let output = Command::new("rustc")
        .arg(&temp_rs_path)
        .arg("-Z")
        .arg("ast-json")
        .arg("--crate-name")
        .arg("temp_ast")
        .arg("--color")
        .arg("never")
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value = serde_json::from_str(&stdout)?;
        Ok(json)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!("Failed to get AST JSON: {}", stderr))
    }
}
/// 获取 Rust 版本信息
pub fn get_rust_version() -> Result<String> {
    let output = Command::new("rustc").arg("--version").output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Ok("Unknown".to_string())
    }
}