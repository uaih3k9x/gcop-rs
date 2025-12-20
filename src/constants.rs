//! 全局常量定义

/// LLM 相关常量
pub mod llm {
    /// 默认 max_tokens
    pub const DEFAULT_MAX_TOKENS: u32 = 2000;

    /// 默认 temperature
    pub const DEFAULT_TEMPERATURE: f32 = 0.3;
}

/// HTTP 相关常量
pub mod http {
    /// 请求超时时间（秒）
    pub const REQUEST_TIMEOUT_SECS: u64 = 120;

    /// 连接超时时间（秒）
    pub const CONNECT_TIMEOUT_SECS: u64 = 10;
}

/// Commit 相关常量
pub mod commit {
    /// 最大重试次数
    pub const MAX_RETRIES: usize = 10;
}

/// UI 相关常量
pub mod ui {
    /// 错误预览最大长度
    pub const ERROR_PREVIEW_LENGTH: usize = 500;

    /// 用户反馈最大长度
    pub const MAX_FEEDBACK_LENGTH: usize = 200;
}

/// 文件相关常量
pub mod file {
    /// 最大文件大小（10MB）
    pub const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;
}

/// 默认的 commit prompt 模板
pub mod prompts {
    /// 默认的 commit prompt 模板
    pub const DEFAULT_COMMIT_PROMPT: &str = r#"You are an expert software engineer reviewing a git diff to generate a concise, informative commit message.

    ## Git Diff:
    ```
    {diff}
    ```

    ## Context:
    - Files changed: {files_changed}
    - Insertions: {insertions}
    - Deletions: {deletions}
    {branch_info}

    ## Instructions:
    1. Analyze the changes carefully
    2. Generate a commit message following conventional commits format
    3. First line: type(scope): brief summary (max 72 chars)
    4. Blank line
    5. Body: explain what and why (not how), if necessary
    6. Keep it concise but informative

    Common types: feat, fix, docs, style, refactor, test, chore

    Output only the commit message, no explanations."#;

    /// 默认的 review prompt 模板
    pub const DEFAULT_REVIEW_PROMPT: &str = r#"You are an expert code reviewer. Review the following code changes carefully.

    ## Code to Review:
    ```
    {diff}
    ```

    ## Review Criteria:
    1. **Correctness**: Are there any bugs or logical errors?
    2. **Security**: Are there any security vulnerabilities?
    3. **Performance**: Are there any performance issues?
    4. **Maintainability**: Is the code readable and maintainable?
    5. **Best Practices**: Does it follow best practices?"#;

    /// 默认的 JSON 输出格式说明（用于自定义 review prompt 时追加）
    pub const DEFAULT_JSON_FORMAT: &str = r#"## Output Format:
    Provide your review in JSON format
    Do not include any explanations outside the JSON structure. Format as follows:
    {{
    "summary": "Brief overall assessment",
    "issues": [
        {{
        "severity": "critical" | "warning" | "info",
        "description": "Issue description",
        "file": "filename (if applicable)",
        "line": line_number (if applicable)
        }}
    ],
    "suggestions": [
        "Improvement suggestion 1"
    ]
    }}

    If no issues found, return empty issues array but provide constructive suggestions."#;
}
