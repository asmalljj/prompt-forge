use reqwest::Client;
use serde::{Deserialize, Serialize};

/// DeepSeek API 请求
#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

/// 消息结构
#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

/// DeepSeek API 响应
#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

/// 响应选项
#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

/// 调用 DeepSeek API
pub async fn call_api(
    api_key: &str,
    base_url: &str,
    model: &str,
    system_prompt: &str,
    user_input: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = ChatRequest {
        model: model.to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_input.to_string(),
            },
        ],
        stream: false,
    };

    let url = format!("{}/chat/completions", base_url);

    let resp = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("API 请求失败: {} - {}", status, body).into());
    }

    let response: ChatResponse = resp.json().await?;

    response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| "API 返回为空".into())
}
