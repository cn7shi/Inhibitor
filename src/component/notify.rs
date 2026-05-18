use reqwest::Client;
use serde_json::json;
use tracing::{error, info, warn};

pub struct Notify;

impl Notify {
    /// 飞书 (Feishu) Webhook 通知
    pub async fn send_feishu(webhook_url: &str, title: &str, message: &str) {
        if webhook_url.is_empty() {
            return;
        }

        let client = Client::new();
        let payload = json!({
            "msg_type": "text",
            "content": {
                "text": format!("【{}】\n{}", title, message)
            }
        });

        match client.post(webhook_url).json(&payload).send().await {
            Ok(res) => {
                let status = res.status();
                let body = res.text().await.unwrap_or_default();
                if !status.is_success() || body.contains("\"code\":") && !body.contains("\"code\":0") || body.contains("\"StatusCode\":") && !body.contains("\"StatusCode\":0") {
                    error!(status = %status, response = %body, "飞书通知发送失败");
                } else {
                    info!("飞书通知发送成功: {}", body);
                }
            }
            Err(e) => {
                error!(error = %e, "飞书通知请求异常");
            }
        }
    }

    /// 钉钉 (DingTalk) Webhook 通知
    pub async fn send_dingtalk(webhook_url: &str, title: &str, message: &str) {
        if webhook_url.is_empty() {
            return;
        }

        let client = Client::new();
        let payload = json!({
            "msgtype": "text",
            "text": {
                "content": format!("【{}】\n{}", title, message)
            }
        });

        match client.post(webhook_url).json(&payload).send().await {
            Ok(res) => {
                let status = res.status();
                let body = res.text().await.unwrap_or_default();
                if !status.is_success() || body.contains("\"errcode\":") && !body.contains("\"errcode\":0") || body.contains("\"code\":") && !body.contains("\"code\":0") {
                    error!(status = %status, response = %body, "钉钉通知发送失败");
                } else {
                    info!("钉钉通知发送成功: {}", body);
                }
            }
            Err(e) => {
                error!(error = %e, "钉钉通知请求异常");
            }
        }
    }

    /// 统一发送通知入口（从配置文件获取 Webhook URL）
    pub async fn send(title: &str, message: &str) {
        use crate::config::Config;
        let config = match Config::load() {
            Ok(c) => c,
            Err(e) => {
                warn!("加载配置失败，跳过通知发送: {}", e);
                return;
            }
        };

        let feishu_url = config.feishu_webhook;
        let dingtalk_url = config.dingtalk_webhook;
        let keyword = config.notify_keyword;
        
        let mut sent = false;

        // 如果配置了自定义关键词，我们把它拼接到标题前面
        let final_title = if !keyword.is_empty() {
            format!("{} {}", keyword, title)
        } else {
            title.to_string()
        };

        if !feishu_url.is_empty() {
            Self::send_feishu(&feishu_url, &final_title, message).await;
            sent = true;
        }
        
        if !dingtalk_url.is_empty() {
            Self::send_dingtalk(&dingtalk_url, &final_title, message).await;
            sent = true;
        }
        
        if !sent {
            warn!("未配置通知 Webhook (feishu_webhook / dingtalk_webhook)，跳过发送: {} - {}", title, message);
        }
    }
}
