use crate::error::ApiResponse;
use crate::settings;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

/// 设置响应
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsResponse {
    /// 是否已配置 API Key（不返回 Key 本身，避免泄露）
    pub has_api_key: bool,
}

/// 保存设置请求
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveSettingsRequest {
    /// DeepSeek API Key
    pub deepseek_api_key: String,
}

/// 获取设置状态（是否已配置 API Key）
#[tauri::command]
pub async fn get_settings(app: AppHandle) -> Result<ApiResponse<SettingsResponse>, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("获取配置目录失败: {}", e))?;

    log::info!("get_settings: config_dir = {:?}", config_dir);

    let settings = settings::load(&config_dir);

    log::info!(
        "get_settings: has_api_key = {:?}",
        settings.deepseek_api_key.as_deref().map(|k| !k.is_empty())
    );

    Ok(ApiResponse::success(SettingsResponse {
        has_api_key: settings
            .deepseek_api_key
            .map(|k| !k.is_empty())
            .unwrap_or(false),
    }))
}

/// 保存 API Key 设置
#[tauri::command]
pub async fn save_settings(
    app: AppHandle,
    request: SaveSettingsRequest,
) -> Result<ApiResponse<()>, String> {
    let key = request.deepseek_api_key.trim();

    if key.is_empty() {
        return Ok(ApiResponse::error("API Key 不能为空"));
    }

    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("获取配置目录失败: {}", e))?;

    let settings = settings::Settings {
        deepseek_api_key: Some(key.to_string()),
    };

    settings::save(&config_dir, &settings)?;

    log::info!("API Key 已保存到本地配置");
    Ok(ApiResponse::success(()))
}
