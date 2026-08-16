use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// 应用设置（本地保存，用户可在应用内配置）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    /// DeepSeek API Key（用户应用内配置，优先级高于 .env）
    pub deepseek_api_key: Option<String>,
}

/// 设置文件名
const SETTINGS_FILE: &str = "settings.json";

/// 获取设置文件路径（app config 目录）
pub fn settings_path(app_config_dir: &std::path::Path) -> PathBuf {
    app_config_dir.join(SETTINGS_FILE)
}

/// 读取设置（文件不存在时返回默认设置）
pub fn load(app_config_dir: &std::path::Path) -> Settings {
    let path = settings_path(app_config_dir);

    if !path.exists() {
        return Settings::default();
    }

    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Settings::default(),
    }
}

/// 保存设置（写入 app config 目录）
pub fn save(app_config_dir: &std::path::Path, settings: &Settings) -> Result<(), String> {
    let path = settings_path(app_config_dir);

    // 确保目录存在
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建配置目录失败: {}", e))?;
    }

    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("序列化设置失败: {}", e))?;

    fs::write(&path, json).map_err(|e| format!("保存设置失败: {}", e))
}
