// src/config.rs
// 配置读取：从 config/config.toml 加载项目常量

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub groq_api_key: String,
    pub model: String,
    pub max_retries: u32,
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
}
