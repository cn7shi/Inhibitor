// src/config.rs
// 配置读取：从 config/config.toml 加载项目常量

use std::collections::HashMap;
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

    /// SAN 值热更新覆盖表。
    /// key 为错误标识符（如 "json::empty"），value 为覆盖后的最终扣分值。
    /// 默认为空 —— 不配任何东西系统就用代码里 trait 的默认值。
    /// 运维发现某个扣分值不合理时，加一行即可立刻生效（热更新），无需重新编译。
    #[serde(default)]
    pub san_overrides: HashMap<String, i32>,
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
