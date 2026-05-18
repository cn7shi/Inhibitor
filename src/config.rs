// src/config.rs
// 配置读取：从 config/config.toml 加载项目常量

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub groq_api_key: String,
    pub model: String,
    pub max_retries: u32,
    #[serde(default)]
    pub feishu_webhook: String,
    #[serde(default)]
    pub dingtalk_webhook: String,
    #[serde(default)]
    pub discord_webhook: String,
    #[serde(default)]
    pub notify_keyword: String,
}

impl Config {
    /// 从 config/config.toml 读取配置
    pub fn load() -> Result<Config, String> {
        let content = std::fs::read_to_string("config/config.toml")
            .map_err(|e| format!("读取 config/config.toml 失败: {}", e))?;

        let config: Config = toml::from_str(&content)
            .map_err(|e| format!("解析 config.toml 失败: {}", e))?;

        Ok(config)
    }

    /// 将配置写回 config/config.toml
    pub fn save(&self) -> Result<(), String> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("序列化配置失败: {}", e))?;
        
        std::fs::write("config/config.toml", content)
            .map_err(|e| format!("写入 config.toml 失败: {}", e))?;
            
        Ok(())
    }
}
