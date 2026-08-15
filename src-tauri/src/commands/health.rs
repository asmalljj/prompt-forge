use crate::error::ApiResponse;
use crate::state::AppState;
use serde::Serialize;
use tauri::State;

/// 健康检查响应
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub request_count: u64,
}

/// 健康检查接口
/// 用于验证后端服务是否正常运行
#[tauri::command]
pub async fn check_health(state: State<'_, AppState>) -> Result<ApiResponse<HealthResponse>, String> {
    // 增加请求计数
    state.increment_request_count();

    let response = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        request_count: state.get_request_count(),
    };

    Ok(ApiResponse::success(response))
}
