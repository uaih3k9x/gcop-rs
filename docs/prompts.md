# Custom Prompts

gcop-rs allows you to customize the prompts sent to AI for both commit message generation and code review.

## Why Customize Prompts?

- **Language**: Generate commit messages in Chinese or other languages
- **Style**: Match your team's commit message format
- **Focus**: Emphasize specific review criteria (security, performance, etc.)
- **Context**: Add project-specific guidelines

## Template Variables

### Commit Message Prompts

Available placeholders:

- `{diff}` - Complete git diff content
- `{files_changed}` - Comma-separated list of changed files
- `{insertions}` - Number of lines added
- `{deletions}` - Number of lines deleted
- `{branch_name}` - Current branch name (if available)
- `{branch_info}` - Formatted branch info (`"- Branch: xxx"` or empty)

### Code Review Prompts

Available placeholders:

- `{diff}` - Code diff content

## Examples

### Chinese Commit Messages

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

### Simple Commit Messages

```toml
[commit]
custom_prompt = """
Generate a one-line commit message for:

{diff}

Files: {files_changed} (+{insertions} -{deletions})

Format: <type>: <description>
Keep it under 72 characters.
"""
```

### Security-Focused Review

```toml
[review]
custom_prompt = """
You are a security expert. Review this code for vulnerabilities:

{diff}

Focus on:
1. SQL Injection
2. XSS (Cross-Site Scripting)
3. CSRF
4. Authentication/Authorization flaws
5. Insecure data handling

Output in JSON format:
{{
  "summary": "Security assessment",
  "issues": [
    {{
      "severity": "critical" | "warning" | "info",
      "description": "Issue description",
      "file": "filename",
      "line": line_number
    }}
  ],
  "suggestions": ["Security recommendation"]
}}
"""
```

### Performance-Focused Review

```toml
[review]
custom_prompt = """
Review this code for performance issues:

{diff}

Check for:
- Inefficient algorithms
- Memory leaks
- Unnecessary allocations
- N+1 queries
- Blocking operations

Return JSON with findings.
"""
```

## Best Practices

### 1. Keep Prompts Focused

Shorter, focused prompts often work better than very long ones.

### 2. Specify Output Format

For code review, always request JSON format to ensure proper parsing.

### 3. Test with Verbose Mode

Use `gcop -v commit` to see the actual prompt sent to AI.

### 4. Iterate

Try different prompts and compare results to find what works best.

## Default Prompts

If you don't specify `custom_prompt`, gcop-rs uses built-in defaults:

- **Commit**: Generates conventional commit messages in English
- **Review**: Comprehensive code review in English with JSON output

See `src/llm/prompt.rs` in the source code for the exact default templates.

## Troubleshooting

### Issue: LLM Returns Wrong Format

If the AI doesn't follow your prompt format:

1. Be more explicit about the required format
2. Add examples in the prompt
3. Use `--verbose` to debug
4. Try a different model or adjust temperature

### Issue: Non-English Output

Some models may ignore language instructions. Try:

- Being more explicit: "You MUST respond in Chinese"
- Adding examples
- Using a different model

## See Also

- [Configuration Reference](configuration.md) - All config options
- [Troubleshooting](troubleshooting.md) - Common issues
