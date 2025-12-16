# 配置指南

## 配置文件位置

gcop-rs 使用 TOML 配置文件，位置：

```
~/.config/gcop/config.toml
```

配置文件是**可选的**。如果不存在，将使用默认值。

## 快速设置

复制示例配置：

```bash
mkdir -p ~/.config/gcop
cp examples/config.toml.example ~/.config/gcop/config.toml
```

然后编辑 `~/.config/gcop/config.toml` 添加你的 API key。

## 基础配置

使用 Claude API 的最小配置：

```toml
[llm]
default_provider = "claude"

[llm.providers.claude]
api_key = "sk-ant-your-key-here"
model = "claude-sonnet-4-5-20250929"
```

## 完整配置示例

```toml
# LLM 配置
[llm]
default_provider = "claude"

# Claude Provider
[llm.providers.claude]
api_key = "sk-ant-your-key"
endpoint = "https://api.anthropic.com/v1/messages"
model = "claude-sonnet-4-5-20250929"
temperature = 0.3
max_tokens = 2000

# OpenAI Provider
[llm.providers.openai]
api_key = "sk-your-openai-key"
endpoint = "https://api.openai.com/v1/chat/completions"
model = "gpt-4-turbo"
temperature = 0.3

# Ollama Provider（本地）
[llm.providers.ollama]
endpoint = "http://localhost:11434/api/generate"
model = "codellama:13b"

# Commit 行为
[commit]
show_diff_preview = true
allow_edit = true
confirm_before_commit = true

# Review 设置
[review]
show_full_diff = true
min_severity = "info"  # critical | warning | info

# UI 设置
[ui]
colored = true
verbose = false
```

## 配置选项

### LLM 设置

| 选项 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `default_provider` | String | `"claude"` | 默认使用的 LLM provider |

### Provider 设置

每个 `[llm.providers.<name>]` 下的 provider 支持：

| 选项 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `api_style` | String | 否 | API 风格：`"claude"`、`"openai"` 或 `"ollama"`（未设置时自动检测） |
| `api_key` | String | 是* | API key（*Ollama 不需要） |
| `endpoint` | String | 否 | API 端点（未设置时使用默认值） |
| `model` | String | 是 | 模型名称 |
| `temperature` | Float | 否 | 温度参数（0.0-1.0，默认: 0.3） |
| `max_tokens` | Integer | 否 | 最大 token 数（仅 Claude，默认: 2000） |

### Commit 设置

| 选项 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `show_diff_preview` | Boolean | `true` | 生成前显示 diff 统计 |
| `allow_edit` | Boolean | `true` | 允许编辑生成的消息 |
| `confirm_before_commit` | Boolean | `true` | 提交前要求确认 |
| `custom_prompt` | String | 无 | 自定义提交信息生成的 prompt 模板 |

### Review 设置

| 选项 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `show_full_diff` | Boolean | `true` | 审查时显示完整 diff |
| `min_severity` | String | `"info"` | 最低显示的严重性：`"critical"`、`"warning"` 或 `"info"` |
| `custom_prompt` | String | 无 | 自定义代码审查的 prompt 模板 |

### UI 设置

| 选项 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `colored` | Boolean | `true` | 启用彩色输出 |
| `verbose` | Boolean | `false` | 显示详细日志（等同于 `--verbose` 标志） |

## API Key 配置

### 优先级顺序

1. **配置文件**（`~/.config/gcop/config.toml`）
2. **环境变量**（fallback）

### 配置方式

**方式 1: 配置文件（推荐）**

```toml
[llm.providers.claude]
api_key = "sk-ant-your-key"
```

**方式 2: 环境变量**

```bash
export ANTHROPIC_API_KEY="sk-ant-your-key"
export OPENAI_API_KEY="sk-your-openai-key"
```

### 安全建议

- **设置文件权限**: `chmod 600 ~/.config/gcop/config.toml`
- **不要提交** config.toml 到 git
- **添加到 .gitignore**（如果创建项目级配置）

## 命令行覆盖

```bash
# 覆盖 provider
gcop-rs --provider openai commit

# 启用详细模式
gcop-rs -v commit
```

命令行选项优先级高于配置文件。

## 参考

- [Provider 设置](providers.md) - 配置 LLM 提供商
- [自定义 Prompt](prompts.md) - 自定义 AI prompts
- [故障排除](troubleshooting.md) - 常见配置问题
