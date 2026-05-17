// src/component/groq_test.rs
// 简单测试：调用 Groq API，跑通一个最小请求

pub struct GroqTest;

impl GroqTest {
    /// 简单对话（无工具）
    pub async fn call(user_message: &str) -> Result<String, String> {
        let cfg = crate::config::Config::load()?;

        let body = serde_json::json!({
            "model": cfg.model,
            "messages": [{ "role": "user", "content": user_message }],
            "temperature": 1,
            "max_completion_tokens": 1024
        });

        let json = Self::send_request(&cfg.groq_api_key, &body).await?;

        let reply = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("(无内容)")
            .to_string();

        Ok(reply)
    }

    /// 工具调用演示：让模型决定是否调用 get_weather
    pub async fn call_with_tools(user_message: &str) -> Result<String, String> {
        let cfg = crate::config::Config::load()?;

        // ===== 第1轮：发送消息 + 工具定义 =====
        let tools = serde_json::json!([
            {
                "type": "function",
                "function": {
                    "name": "get_weather",
                    "description": "Get current weather for a location",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "location": {
                                "type": "string",
                                "description": "City and state, e.g. San Francisco, CA"
                            },
                            "unit": {
                                "type": "string",
                                "enum": ["celsius", "fahrenheit"]
                            }
                        },
                        "required": ["location"]
                    }
                }
            }
        ]);

        let messages = serde_json::json!([
            {
                "role": "system",
                "content": "You are a weather assistant. Respond to the user question and use tools if needed."
            },
            {
                "role": "user",
                "content": user_message
            }
        ]);

        let body = serde_json::json!({
            "model": cfg.model,
            "tools": tools,
            "messages": messages,
            "max_completion_tokens": 1024
        });

        tracing::info!("[工具调用] 第1轮：发送消息 + 工具定义...");
        let json = Self::send_request(&cfg.groq_api_key, &body).await?;

        // 检查模型是否要调用工具
        let choice = &json["choices"][0]["message"];
        let tool_calls = &choice["tool_calls"];

        if tool_calls.is_null() || !tool_calls.is_array() {
            // 模型没有调用工具，直接返回文本回复
            let reply = choice["content"].as_str().unwrap_or("(无内容)");
            return Ok(format!("(模型未调用工具，直接回复) {}", reply));
        }

        // ===== 解析工具调用 =====
        let tool_call = &tool_calls[0];
        let call_id = tool_call["id"].as_str().unwrap_or("");
        let func_name = tool_call["function"]["name"].as_str().unwrap_or("");
        let func_args = tool_call["function"]["arguments"].as_str().unwrap_or("{}");

        tracing::info!("[工具调用] 模型请求调用: {}({})", func_name, func_args);

        // ===== 执行本地函数 =====
        let tool_result = match func_name {
            "get_weather" => Self::mock_get_weather(func_args),
            _ => format!("未知函数: {}", func_name),
        };
        tracing::info!("[工具调用] 本地执行结果: {}", tool_result);

        // ===== 第2轮：把工具结果发回给模型 =====
        let mut round2_messages = messages.as_array().unwrap().clone();
        // 加上 assistant 的工具调用消息
        round2_messages.push(choice.clone());
        // 加上工具执行结果
        round2_messages.push(serde_json::json!({
            "role": "tool",
            "tool_call_id": call_id,
            "content": tool_result
        }));

        let body2 = serde_json::json!({
            "model": cfg.model,
            "messages": round2_messages,
            "max_completion_tokens": 1024
        });

        tracing::info!("[工具调用] 第2轮：把工具结果发回给模型...");
        let json2 = Self::send_request(&cfg.groq_api_key, &body2).await?;

        let final_reply = json2["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("(无内容)")
            .to_string();

        Ok(final_reply)
    }

    // ===== 内部工具函数 =====

    /// 模拟天气查询（真实场景换成真的 API）
    fn mock_get_weather(args_json: &str) -> String {
        let args: serde_json::Value = serde_json::from_str(args_json).unwrap_or_default();
        let location = args["location"].as_str().unwrap_or("unknown");
        // 返回一个模拟结果
        format!("{{\"location\": \"{}\", \"temperature\": 22, \"unit\": \"celsius\", \"condition\": \"sunny\"}}", location)
    }

    /// 公共的发请求逻辑，避免重复代码
    async fn send_request(api_key: &str, body: &serde_json::Value) -> Result<serde_json::Value, String> {
        let resp = reqwest::Client::new()
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(body)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        let status = resp.status();
        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("解析失败: {}", e))?;

        if !status.is_success() {
            let err_msg = json["error"]["message"].as_str().unwrap_or("未知错误");
            return Err(format!("API 错误 (HTTP {}): {}", status, err_msg));
        }

        // 记录发送的 JSON 数据
        tracing::info!("===== 发送给 LLM 的 JSON =====");
        tracing::info!("\n{}", serde_json::to_string_pretty(body).unwrap_or_default());

        // 提取并记录 Token 消耗
        let usage = &json["usage"];
        if !usage.is_null() {
            let prompt = usage["prompt_tokens"].as_i64().unwrap_or(0);
            let completion = usage["completion_tokens"].as_i64().unwrap_or(0);
            let total = usage["total_tokens"].as_i64().unwrap_or(0);
            tracing::info!(
                prompt_tokens = prompt,
                completion_tokens = completion,
                total_tokens = total,
                "===== Token 消耗 ====="
            );
        }

        Ok(json)
    }
}
