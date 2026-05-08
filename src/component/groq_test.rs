// src/component/groq_test.rs
// 简单测试：调用 Groq API，跑通一个最小请求

pub struct GroqTest;

impl GroqTest {
    pub async fn call(user_message: &str) -> Result<String, String> {
        // 从 config/config.toml 读取配置
        let cfg = crate::config::Config::load()?;

        let body = serde_json::json!({
            "model": cfg.model,
            "messages": [{ "role": "user", "content": user_message }],
            "temperature": 1,
            "max_completion_tokens": 1024
        });

        let resp = reqwest::Client::new()
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", cfg.groq_api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("解析失败: {}", e))?;

        // 直接从 JSON 里取值，不需要定义任何 struct
        let reply = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("(无内容)")
            .to_string();

        Ok(reply)
    }
}
