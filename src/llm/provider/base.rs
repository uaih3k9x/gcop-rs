//! Provider 公共抽象和辅助函数
//!
//! 提取各 Provider 的通用逻辑，减少重复代码

use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::config::ProviderConfig;
use crate::constants::llm::{DEFAULT_MAX_TOKENS, DEFAULT_TEMPERATURE};
use crate::constants::ui::ERROR_PREVIEW_LENGTH;
use crate::error::{GcopError, Result};
use crate::llm::ReviewResult;

use super::utils::complete_endpoint;

/// 发送 LLM API 请求的通用函数
///
/// # Arguments
/// * `client` - HTTP 客户端
/// * `endpoint` - API 端点
/// * `headers` - 额外的请求头
/// * `request_body` - 请求体
/// * `provider_name` - Provider 名称（用于日志和错误信息）
pub async fn send_llm_request<Req, Resp>(
    client: &Client,
    endpoint: &str,
    headers: &[(&str, &str)],
    request_body: &Req,
    provider_name: &str,
) -> Result<Resp>
where
    Req: Serialize,
    Resp: DeserializeOwned,
{
    let mut req = client
        .post(endpoint)
        .header("Content-Type", "application/json");

    for (key, value) in headers {
        req = req.header(*key, *value);
    }

    tracing::debug!("Sending request to: {}", endpoint);

    let response = req.json(request_body).send().await.map_err(|e| {
        let error_details = format!("{}", e);
        let mut error_type = "unknown";

        if e.is_timeout() {
            error_type = "timeout";
        } else if e.is_connect() {
            error_type = "connection failed";
        } else if e.is_request() {
            error_type = "request error";
        } else if e.is_body() {
            error_type = "body error";
        } else if e.is_decode() {
            error_type = "decode error";
        }

        tracing::error!("{} API request failed [{}]: {}", provider_name, error_type, error_details);

        // 为不同类型的网络错误提供更详细的错误信息
        if e.is_timeout() {
            GcopError::Llm(format!(
                "{} API request timeout: {}. The request took too long to complete.",
                provider_name, error_details
            ))
        } else if e.is_connect() {
            GcopError::Llm(format!(
                "{} API connection failed: {}. Check network connectivity or API endpoint.",
                provider_name, error_details
            ))
        } else {
            GcopError::Network(e)
        }
    })?;

    let status = response.status();
    let response_text = response.text().await?;

    tracing::debug!("{} API response status: {}", provider_name, status);
    tracing::debug!("{} API response body: {}", provider_name, response_text);

    if !status.is_success() {
        return Err(GcopError::Llm(format!(
            "{} API error ({}): {}",
            provider_name, status, response_text
        )));
    }

    serde_json::from_str(&response_text).map_err(|e| {
        GcopError::Llm(format!(
            "Failed to parse {} response: {}. Raw response: {}",
            provider_name, e, response_text
        ))
    })
}

/// 提取 API key（配置优先，环境变量 fallback）
///
/// # Arguments
/// * `config` - Provider 配置
/// * `env_var` - 环境变量名
/// * `provider_name` - Provider 名称（用于错误提示）
pub fn extract_api_key(
    config: &ProviderConfig,
    env_var: &str,
    provider_name: &str,
) -> Result<String> {
    config
        .api_key
        .clone()
        .or_else(|| std::env::var(env_var).ok())
        .ok_or_else(|| {
            GcopError::Config(format!(
                "{} API key not found. Set api_key in config.toml or {} environment variable",
                provider_name, env_var
            ))
        })
}

/// 构建完整 endpoint
///
/// # Arguments
/// * `config` - Provider 配置
/// * `default_base` - 默认 base URL
/// * `suffix` - API 路径后缀
pub fn build_endpoint(config: &ProviderConfig, default_base: &str, suffix: &str) -> String {
    config
        .endpoint
        .as_ref()
        .map(|e| complete_endpoint(e, suffix))
        .unwrap_or_else(|| format!("{}{}", default_base, suffix))
}

/// 提取 extra 配置中的 u32 值
pub fn extract_extra_u32(config: &ProviderConfig, key: &str) -> Option<u32> {
    config
        .extra
        .get(key)
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
}

/// 提取 extra 配置中的 u32 值，带默认值
pub fn extract_extra_u32_or(config: &ProviderConfig, key: &str, default: u32) -> u32 {
    extract_extra_u32(config, key).unwrap_or(default)
}

/// 提取 extra 配置中的 f32 值
pub fn extract_extra_f32(config: &ProviderConfig, key: &str) -> Option<f32> {
    config
        .extra
        .get(key)
        .and_then(|v| v.as_f64())
        .map(|v| v as f32)
}

/// 提取 extra 配置中的 f32 值，带默认值
pub fn extract_extra_f32_or(config: &ProviderConfig, key: &str, default: f32) -> f32 {
    extract_extra_f32(config, key).unwrap_or(default)
}

/// 获取默认的 max_tokens
pub fn default_max_tokens() -> u32 {
    DEFAULT_MAX_TOKENS
}

/// 获取默认的 temperature
pub fn default_temperature() -> f32 {
    DEFAULT_TEMPERATURE
}

/// 清理 JSON 响应（移除 markdown 代码块标记）
pub fn clean_json_response(response: &str) -> &str {
    let trimmed = response.trim();

    // 提取 { 到 } 之间的内容
    if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}'))
        && start < end
    {
        return &trimmed[start..=end];
    }

    // Backup: 回退到移除 markdown 代码块标记
    let without_prefix = trimmed
        .strip_prefix("```json")
        .or_else(|| trimmed.strip_prefix("```JSON"))
        .or_else(|| trimmed.strip_prefix("```"))
        .map(|s| s.trim_start()) // 移除前缀后的换行符
        .unwrap_or(trimmed);

    without_prefix
        .strip_suffix("```")
        .map(|s| s.trim_end()) // 移除后缀前的换行符
        .unwrap_or(without_prefix)
        .trim()
}

/// 截断字符串用于错误预览
pub fn truncate_for_preview(s: &str) -> String {
    if s.len() > ERROR_PREVIEW_LENGTH {
        format!("{}...", &s[..ERROR_PREVIEW_LENGTH])
    } else {
        s.to_string()
    }
}

/// 解析 review 响应 JSON
pub fn parse_review_response(response: &str) -> Result<ReviewResult> {
    let cleaned = clean_json_response(response);
    serde_json::from_str(cleaned).map_err(|e| {
        let preview = truncate_for_preview(response);
        GcopError::Llm(format!(
            "Failed to parse review result: {}. Response preview: {}",
            e, preview
        ))
    })
}
