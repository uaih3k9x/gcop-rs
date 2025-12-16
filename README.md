# gcop-rs

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

AI-powered Git commit message generator and code reviewer, written in Rust.

**[中文文档](README_ZH.md)** | **[Documentation](docs/)**

## Features

- 🤖 **AI Commit Messages** - Generate conventional commit messages using Claude, OpenAI, or Ollama
- 🔍 **Code Review** - Get AI-powered code reviews with security and performance insights
- 🔧 **Custom Providers** - Support any OpenAI/Claude compatible API (DeepSeek, custom endpoints, etc.)
- 📝 **Custom Prompts** - Customize generation and review prompts with template variables
- ⚙️  **Flexible Config** - Configure via file or environment variables
- 🎨 **Beautiful CLI** - Spinner animations, colored output, and interactive prompts
- 🐛 **Debug Mode** - Verbose logging with full request/response inspection

## Quick Start

### 1. Installation

```bash
# Clone and build
git clone https://github.com/your-repo/gcop-rs.git
cd gcop-rs
cargo build --release

# Copy to PATH
sudo cp target/release/gcop-rs /usr/local/bin/gcop-rs
```

See [docs/installation.md](docs/installation.md) for more options.

### 2. Configure

Create `~/.config/gcop/config.toml`:

```toml
[llm]
default_provider = "claude"

[llm.providers.claude]
api_key = "sk-ant-your-key-here"
model = "claude-sonnet-4-5-20250929"
```

Or use environment variables:
```bash
export ANTHROPIC_API_KEY="sk-ant-your-key"
```

See [docs/configuration.md](docs/configuration.md) for all options.

### 3. Use

```bash
# Generate commit message
git add .
gcop-rs commit

# Review uncommitted changes
gcop-rs review changes

# Review a specific commit
gcop-rs review commit abc123

# Use different provider
gcop-rs --provider openai commit
```

## Commands

### `gcop-rs commit`

Generate commit message for staged changes.

```bash
gcop-rs commit              # Generate, edit, and commit
gcop-rs commit --no-edit    # Skip editor
gcop-rs commit --yes        # Skip confirmation
gcop-rs -v commit           # Verbose mode
```

### `gcop-rs review`

Review code changes with AI.

```bash
gcop-rs review changes           # Review uncommitted changes
gcop-rs review commit <hash>     # Review a commit
gcop-rs review range main..dev   # Review commit range
gcop-rs review file src/main.rs  # Review a file
```

**Output formats**: `--format text|json|markdown`

## Configuration

Config file location: `~/.config/gcop/config.toml`

Example configuration with Claude API:

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

For complete configuration reference, see [docs/configuration.md](docs/configuration.md).

## Advanced Features

### Custom Providers

Add any OpenAI or Claude compatible API:

```toml
[llm.providers.deepseek]
api_style = "openai"
api_key = "sk-your-deepseek-key"
endpoint = "https://api.deepseek.com/v1/chat/completions"
model = "deepseek-chat"
```

See [docs/providers.md](docs/providers.md) for more examples.

### Custom Prompts

Customize commit message or review prompts:

```toml
[commit]
custom_prompt = """
Generate a commit message in Chinese for:
{diff}

Files: {files_changed}
Stats: +{insertions} -{deletions}
"""
```

See [docs/prompts.md](docs/prompts.md) for template variables and examples.

### Debug Mode

Use `--verbose` to see detailed logs:

```bash
gcop-rs -v commit  # Shows API requests, responses, and prompts
```

## Documentation

- **[Installation Guide](docs/installation.md)** - Detailed installation instructions
- **[Configuration Reference](docs/configuration.md)** - Complete configuration guide
- **[Provider Setup](docs/providers.md)** - Configure LLM providers
- **[Custom Prompts](docs/prompts.md)** - Customize AI prompts
- **[Troubleshooting](docs/troubleshooting.md)** - Common issues and solutions

## Requirements

- Rust 1.70 or higher
- Git
- API key for at least one provider (Claude, OpenAI, or local Ollama)

## License

MIT License - see LICENSE file for details.

## Author

AptS-1547 <apts-1547@esaps.net>

---

**Tip**: Run `gcop-rs commit --help` or `gcop-rs review --help` for more options.
