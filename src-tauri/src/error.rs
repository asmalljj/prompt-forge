use serde::Serialize;

/// 统一错误类型
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("配置错误: {0}")]
    Config(String),

    #[error("API 请求错误: {0}")]
    ApiRequest(String),

    #[error("API 响应错误: {0}")]
    ApiResponse(String),

    #[error("序列化错误: {0}")]
    Serialization(String),

    #[error("未知错误: {0}")]
    Unknown(String),
}

/// 统一响应格式
#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    /// 成功响应
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// 错误响应
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

/// 为 AppError 实现 Serialize，以便通过 IPC 传递
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

/// 将 AppError 转换为统一错误响应
impl From<AppError> for ApiResponse<()> {
    fn from(err: AppError) -> Self {
        ApiResponse::error(err.to_string())
    }
}
