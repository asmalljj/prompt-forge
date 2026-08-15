use std::sync::Mutex;

/// 应用状态
pub struct AppState {
    /// 请求计数器（示例）
    request_count: Mutex<u64>,
}

impl AppState {
    /// 创建新的应用状态
    pub fn new() -> Self {
        Self {
            request_count: Mutex::new(0),
        }
    }

    /// 增加请求计数
    pub fn increment_request_count(&self) {
        let mut count = self.request_count.lock().unwrap();
        *count += 1;
    }

    /// 获取请求计数
    pub fn get_request_count(&self) -> u64 {
        *self.request_count.lock().unwrap()
    }
}
