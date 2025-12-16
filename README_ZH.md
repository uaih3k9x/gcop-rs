# gcop-rs

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

AI 驱动的 Git 提交信息生成器和代码审查工具，使用 Rust 编写。

**[English](README.md)** | **[文档](docs/zh/)**

## 功能特性

- 🤖 **AI 生成提交信息** - 使用 Claude、OpenAI 或 Ollama 生成符合规范的提交信息
- 🔍 **代码审查** - AI 驱动的代码审查，关注安全性和性能问题
- 🔧 **自定义 Provider** - 支持任意 OpenAI/Claude 兼容的 API（DeepSeek、自定义端点等）
- 📝 **自定义 Prompt** - 使用模板变量自定义生成和审查的 prompt
- ⚙️  **灵活配置** - 通过配置文件或环境变量配置
- 🎨 **精美界面** - Spinner 动画、彩色输出、交互式提示
- 🐛 **调试模式** - 详细日志，可查看完整的请求/响应

## 快速开始

### 1. 安装

```bash
# 克隆并编译
git clone https://github.com/your-repo/gcop-rs.git
cd gcop-rs
cargo build --release

# 复制到 PATH
sudo cp target/release/gcop-rs /usr/local/bin/gcop-rs
```

详见 [docs/zh/installation.md](docs/zh/installation.md)。

### 2. 配置

创建 `~/.config/gcop/config.toml`：

```toml
[llm]
default_provider = "claude"

[llm.providers.claude]
api_key = "sk-ant-your-key-here"
model = "claude-sonnet-4-5-20250929"
```

或使用环境变量：
```bash
export ANTHROPIC_API_KEY="sk-ant-your-key"
```

详见 [docs/zh/configuration.md](docs/zh/configuration.md)。

### 3. 使用

```bash
# 生成提交信息
git add .
gcop-rs commit

# 审查未提交的变更
gcop-rs review changes

# 审查特定 commit
gcop-rs review commit abc123

# 使用不同的 provider
gcop-rs --provider openai commit
```

## 命令说明

### `gcop-rs commit`

为暂存的变更生成提交信息。

```bash
gcop-rs commit              # 生成、编辑并提交
gcop-rs commit --no-edit    # 跳过编辑器
gcop-rs commit --yes        # 跳过确认
gcop-rs -v commit           # 详细模式
```

### `gcop-rs review`

使用 AI 审查代码变更。

```bash
gcop-rs review changes           # 审查未提交的变更
gcop-rs review commit <hash>     # 审查某个 commit
gcop-rs review range main..dev   # 审查 commit 范围
gcop-rs review file src/main.rs  # 审查某个文件
```

**输出格式**: `--format text|json|markdown`

## 配置

配置文件位置：`~/.config/gcop/config.toml`

使用 Claude API 的示例配置：

```toml
[llm]
default_provider = "claude"

[llm.providers.claude]
api_key = "sk-ant-your-key"
model = "claude-sonnet-4-5-20250929"
temperature = 0.3

[commit]
show_diff_preview = true
allow_edit = true
confirm_before_commit = true

[review]
min_severity = "info"
```

完整配置参考见 [docs/zh/configuration.md](docs/zh/configuration.md)。

## 高级功能

### 自定义 Provider

添加任意 OpenAI 或 Claude 兼容的 API：

```toml
[llm.providers.deepseek]
api_style = "openai"
api_key = "sk-your-deepseek-key"
endpoint = "https://api.deepseek.com/v1/chat/completions"
model = "deepseek-chat"
```

更多示例见 [docs/zh/providers.md](docs/zh/providers.md)。

### 自定义 Prompt

自定义提交信息或审查的 prompt：

```toml
[commit]
custom_prompt = """
为以下变更生成中文提交信息：
{diff}

文件: {files_changed}
统计: +{insertions} -{deletions}
"""
```

模板变量和示例见 [docs/zh/prompts.md](docs/zh/prompts.md)。

### 调试模式

使用 `--verbose` 查看详细日志：

```bash
gcop-rs -v commit  # 显示 API 请求、响应和 prompts
```

## 文档

- **[安装指南](docs/zh/installation.md)** - 详细的安装说明
- **[配置参考](docs/zh/configuration.md)** - 完整的配置指南
- **[Provider 设置](docs/zh/providers.md)** - 配置 LLM 提供商
- **[自定义 Prompt](docs/zh/prompts.md)** - 自定义 AI prompts
- **[故障排除](docs/zh/troubleshooting.md)** - 常见问题和解决方案

## 系统要求

- Rust 1.70 或更高版本
- Git
- 至少一个 provider 的 API key（Claude、OpenAI 或本地 Ollama）

## 许可证

MIT License - 详见 LICENSE 文件。

## 作者

AptS-1547 <apts-1547@esaps.net>

---

**提示**: 运行 `gcop-rs commit --help` 或 `gcop-rs review --help` 查看更多选项。
