use crate::constants::prompts;
use crate::llm::{CommitContext, ReviewType};

/// 构建 commit message 生成的 prompt
pub fn build_commit_prompt(
    diff: &str,
    context: &CommitContext,
    custom_template: Option<&str>,
) -> String {
    let template = match custom_template {
        Some(t) if !t.contains("{diff}") => {
            // 自定义模板缺少 {diff}，追加默认的 diff 部分
            format!(
                "{}\n\n## Git Diff:\n```\n{{diff}}\n```\n\n## Context:\n- Files: {{files_changed}}\n- Changes: +{{insertions}} -{{deletions}}",
                t
            )
        }
        Some(t) => t.to_string(),
        None => prompts::DEFAULT_COMMIT_PROMPT.to_string(),
    };

    let branch_info = context
        .branch_name
        .as_ref()
        .map(|b| format!("- Branch: {}", b))
        .unwrap_or_default();

    let mut prompt = template
        .replace("{diff}", diff)
        .replace("{files_changed}", &context.files_changed.join(", "))
        .replace("{insertions}", &context.insertions.to_string())
        .replace("{deletions}", &context.deletions.to_string())
        .replace(
            "{branch_name}",
            context.branch_name.as_deref().unwrap_or(""),
        )
        .replace("{branch_info}", &branch_info);

    // 在 prompt 尾部追加用户反馈
    if !context.user_feedback.is_empty() {
        prompt.push_str("\n\n## Additional User Requirements:\n");
        for (i, fb) in context.user_feedback.iter().enumerate() {
            prompt.push_str(&format!("{}. {}\n", i + 1, fb));
        }
    }

    prompt
}

/// 构建代码审查的 prompt
pub fn build_review_prompt(
    diff: &str,
    _review_type: &ReviewType,
    custom_template: Option<&str>,
) -> String {
    let mut template = custom_template
        .map(|t| t.to_string())
        .unwrap_or_else(|| prompts::DEFAULT_REVIEW_PROMPT.to_string());

    // 检测并追加缺失的 {diff}
    if !template.contains("{diff}") {
        template.push_str("\n\n## Code to Review:\n```\n{diff}\n```");
    }

    // 始终追加 JSON 格式说明
    template.push_str("\n\n");
    template.push_str(prompts::DEFAULT_JSON_FORMAT);

    template.replace("{diff}", diff)
}
