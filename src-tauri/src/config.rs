use std::sync::OnceLock;

/// 应用配置
#[derive(Debug)]
pub struct AppConfig {
    /// DeepSeek API Key
    pub deepseek_api_key: String,
    /// DeepSeek API 基础 URL
    pub deepseek_base_url: String,
    /// 默认模型
    pub deepseek_model: String,
}

/// 全局配置实例
static CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// 初始化配置（从环境变量读取）
pub fn init() {
    // 尝试加载 .env 文件
    let _ = dotenvy::dotenv();

    let config = AppConfig {
        deepseek_api_key: std::env::var("DEEPSEEK_API_KEY").unwrap_or_default(),
        deepseek_base_url: std::env::var("DEEPSEEK_BASE_URL")
            .unwrap_or_else(|_| "https://api.deepseek.com".to_string()),
        deepseek_model: std::env::var("DEEPSEEK_MODEL")
            .unwrap_or_else(|_| "deepseek-v4-flash".to_string()),
    };

    CONFIG.set(config).expect("config already initialized");
}

/// 获取配置引用
pub fn get() -> &'static AppConfig {
    CONFIG.get().expect("config not initialized")
}
