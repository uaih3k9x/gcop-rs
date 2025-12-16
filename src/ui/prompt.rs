use dialoguer::{Confirm, Input, Select};

use crate::error::{GcopError, Result};

/// 用户对 commit message 的操作选择
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitAction {
    Accept,            // 接受当前 message
    Edit,              // 打开编辑器手动修改
    Retry,             // 重新生成
    RetryWithFeedback, // 重新生成并附带反馈
    Quit,              // 退出
}

/// 显示 commit message 选项菜单
///
/// # Arguments
/// * `_message` - 当前生成的 commit message（暂未使用）
/// * `allow_edit` - 是否允许手动编辑（由配置和 --no-edit 控制）
/// * `retry_count` - 已重试次数（用于显示提示）
///
/// # Returns
/// * `Ok(CommitAction)` - 用户选择的操作
/// * `Err(GcopError::UserCancelled)` - 用户按 Ctrl+C
pub fn commit_action_menu(
    _message: &str,
    allow_edit: bool,
    retry_count: usize,
) -> Result<CommitAction> {
    // 构建选项列表
    let mut options = vec!["✓ Accept - 使用这个 commit message"];

    if allow_edit {
        options.push("✎ Edit - 手动修改 message");
    }

    options.push("↻ Retry - 重新生成");
    options.push("↻+ Retry with feedback - 重新生成并提供指示");
    options.push("✕ Quit - 放弃提交");

    // 根据重试次数调整提示文字
    let prompt = if retry_count == 0 {
        "选择下一步操作:"
    } else {
        "还不满意? 继续选择:"
    };

    let selection = Select::new()
        .with_prompt(prompt)
        .items(&options)
        .default(0) // 默认选择 Accept
        .interact()
        .map_err(|_| GcopError::UserCancelled)?;

    // 映射选择到枚举（需要考虑 allow_edit 的影响）
    let action = if allow_edit {
        match selection {
            0 => CommitAction::Accept,
            1 => CommitAction::Edit,
            2 => CommitAction::Retry,
            3 => CommitAction::RetryWithFeedback,
            4 => CommitAction::Quit,
            _ => unreachable!(),
        }
    } else {
        match selection {
            0 => CommitAction::Accept,
            1 => CommitAction::Retry,
            2 => CommitAction::RetryWithFeedback,
            3 => CommitAction::Quit,
            _ => unreachable!(),
        }
    };

    Ok(action)
}

/// 获取用户对重试的反馈
///
/// # Returns
/// * `Ok(Some(String))` - 用户输入的反馈
/// * `Ok(None)` - 用户未输入或取消
/// * `Err(_)` - 发生错误
pub fn get_retry_feedback() -> Result<Option<String>> {
    println!("\n请提供重新生成的指示（例如: \"使用中文\", \"更简洁\", \"包含更多细节\"）");

    let feedback: String = Input::new()
        .with_prompt("指示")
        .allow_empty(true)
        .interact_text()
        .map_err(|_| GcopError::UserCancelled)?;

    let trimmed = feedback.trim();

    // 限制长度，防止 prompt 过长
    if trimmed.len() > 200 {
        let truncated = &trimmed[..200];
        println!("⚠ 反馈过长，已截断至 200 字符");
        Ok(Some(truncated.to_string()))
    } else if trimmed.is_empty() {
        Ok(None)
    } else {
        Ok(Some(trimmed.to_string()))
    }
}

/// 交互式确认提示
///
/// # Arguments
/// * `message` - 提示信息
/// * `default` - 默认值（true = Yes, false = No）
///
/// # Returns
/// * `Ok(true)` - 用户选择 Yes
/// * `Ok(false)` - 用户选择 No
/// * `Err(_)` - 发生错误
pub fn confirm(message: &str, default: bool) -> Result<bool> {
    let result = Confirm::new()
        .with_prompt(message)
        .default(default)
        .interact()?;

    Ok(result)
}
