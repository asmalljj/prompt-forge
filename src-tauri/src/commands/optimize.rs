use crate::config;
use crate::error::{AppError, ApiResponse};
use crate::services::deepseek;
use serde::{Deserialize, Serialize};
use tauri::State;

/// 优化风格
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OptimizeStyle {
    /// 通用优化
    General,
    /// 简洁风格
    Concise,
    /// 详细风格
    Detailed,
}

/// 优化请求参数
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeRequest {
    /// 原始提示词
    pub input: String,
    /// 优化风格
    pub style: OptimizeStyle,
}

/// 优化响应
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeResponse {
    /// 优化后的提示词
    pub optimized_prompt: String,
    /// 使用的模型
    pub model: String,
}

/// 提示词优化接口
/// 将大白话提示词优化为高质量提示词
#[tauri::command]
pub async fn optimize_prompt(
    request: OptimizeRequest,
    state: State<'_, crate::state::AppState>,
) -> Result<ApiResponse<OptimizeResponse>, AppError> {
    // 增加请求计数
    state.increment_request_count();

    // 获取配置
    let config = config::get();

    // 检查 API Key
    if config.deepseek_api_key.is_empty() {
        return Err(AppError::Config("DEEPSEEK_API_KEY 未配置".to_string()));
    }

    // 构建系统提示词
    let system_prompt = build_system_prompt(&request.style);

    // 调用 DeepSeek API
    let optimized = deepseek::call_api(
        &config.deepseek_api_key,
        &config.deepseek_base_url,
        &config.deepseek_model,
        &system_prompt,
        &request.input,
    )
    .await
    .map_err(|e| AppError::ApiRequest(e.to_string()))?;

    let response = OptimizeResponse {
        optimized_prompt: optimized,
        model: config.deepseek_model.clone(),
    };

    Ok(ApiResponse::success(response))
}

/// 根据优化风格构建系统提示词
fn build_system_prompt(style: &OptimizeStyle) -> String {
    let base_prompt = "你是一个提示词优化专家。用户会给你一段大白话描述的需求，你需要将其优化成 AI 更容易理解和执行的高质量提示词。";

    match style {
        OptimizeStyle::General => {
            format!(
                "{}\n\n优化要求：\n1. 保持用户原意不变\n2. 补充必要的上下文和约束\n3. 结构化输出（角色、任务、要求、格式）\n4. 语言简洁专业",
                base_prompt
            )
        }
        OptimizeStyle::Concise => {
            format!(
                "{}\n\n优化要求：\n1. 保持用户原意不变\n2. 去除所有冗余内容\n3. 只保留核心指令\n4. 用最简短的语言表达",
                base_prompt
            )
        }
        OptimizeStyle::Detailed => {
            format!(
                "{}\n\n优化要求：\n1. 保持用户原意不变\n2. 补充详细的角色设定\n3. 补充完整的任务描述\n4. 补充约束条件和输出格式要求\n5. 补充示例（如果适用）",
                base_prompt
            )
        }
    }
}
