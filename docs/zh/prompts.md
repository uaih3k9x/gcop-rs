# 自定义 Prompt

gcop-rs 允许你自定义发送给 AI 的 prompt，包括提交信息生成和代码审查。

## 为什么要自定义 Prompt？

- **语言**: 生成中文或其他语言的提交信息
- **风格**: 匹配团队的提交信息格式
- **侧重点**: 强调特定的审查标准（安全性、性能等）
- **上下文**: 添加项目特定的指导方针

## 模板变量

### Commit Message Prompts

可用占位符：

- `{diff}` - 完整的 git diff 内容
- `{files_changed}` - 逗号分隔的修改文件列表
- `{insertions}` - 新增行数
- `{deletions}` - 删除行数
- `{branch_name}` - 当前分支名（如果有）
- `{branch_info}` - 格式化的分支信息（`"- Branch: xxx"` 或空字符串）

### Code Review Prompts

可用占位符：

- `{diff}` - 代码 diff 内容

## 示例

### 中文提交信息

```toml
[commit]
custom_prompt = """
你是一个专业的软件工程师，正在审查 git diff 以生成简洁的中文提交信息。

## Git Diff:
{diff}

## 上下文:
- 修改的文件: {files_changed}
- 新增行数: {insertions}
- 删除行数: {deletions}
- 分支名称: {branch_name}

## 要求:
1. 使用中文生成提交信息
2. 第一行：类型(范围): 简要描述（不超过50字）
3. 空一行
4. 正文：说明改动的原因和影响

常见类型：feat（新功能）、fix（修复）、docs（文档）、refactor（重构）

只输出提交信息，不要解释。
"""
```

### 简洁提交信息

```toml
[commit]
custom_prompt = """
为以下变更生成一行提交信息：

{diff}

文件: {files_changed} (+{insertions} -{deletions})

格式: <类型>: <描述>
不超过 72 字符。
"""
```

### 安全性重点审查

```toml
[review]
custom_prompt = """
你是安全专家。审查以下代码的安全漏洞：

{diff}

重点关注：
1. SQL 注入
2. XSS（跨站脚本）
3. CSRF
4. 认证/授权缺陷
5. 不安全的数据处理

以 JSON 格式输出：
{{
  "summary": "安全评估",
  "issues": [
    {{
      "severity": "critical" | "warning" | "info",
      "description": "问题描述",
      "file": "文件名",
      "line": 行号
    }}
  ],
  "suggestions": ["安全建议"]
}}
"""
```

### 性能重点审查

```toml
[review]
custom_prompt = """
审查以下代码的性能问题：

{diff}

检查：
- 低效算法
- 内存泄漏
- 不必要的内存分配
- N+1 查询
- 阻塞操作

返回 JSON 格式的发现。
"""
```

## 最佳实践

### 1. 保持 Prompt 专注

简短、专注的 prompt 通常比很长的 prompt 效果更好。

### 2. 指定输出格式

对于代码审查，始终要求 JSON 格式以确保正确解析。

### 3. 使用详细模式测试

使用 `gcop-rs -v commit` 查看实际发送给 AI 的 prompt。

### 4. 迭代优化

尝试不同的 prompt 并比较结果，找到最适合的。

## 默认 Prompt

如果不指定 `custom_prompt`，gcop-rs 使用内置的默认模板：

- **Commit**: 生成英文的 conventional commit messages
- **Review**: 英文的全面代码审查，JSON 输出

详见源代码中的 `src/llm/prompt.rs`。

## 故障排除

### 问题: LLM 返回错误格式

如果 AI 不遵循你的 prompt 格式：

1. 更明确地说明所需格式
2. 在 prompt 中添加示例
3. 使用 `--verbose` 调试
4. 尝试不同的模型或调整 temperature

### 问题: 非中文输出

某些模型可能忽略语言指令。尝试：

- 更明确：「你必须使用中文回复」
- 添加示例
- 使用不同的模型

## 参考

- [配置参考](configuration.md) - 所有配置选项
- [故障排除](troubleshooting.md) - 常见问题
