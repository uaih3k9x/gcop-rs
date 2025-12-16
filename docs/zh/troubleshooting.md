# 故障排除

gcop-rs 的常见问题和解决方案。

## 安装问题

### 问题: `cargo build` 失败

**解决方案**:
```bash
# 更新 Rust
rustup update

# 清理并重新编译
cargo clean
cargo build --release
```

### 问题: 安装后找不到二进制文件

**解决方案**:
```bash
# 检查二进制文件是否存在
ls -la /usr/local/bin/gcop

# 验证 PATH 包含 /usr/local/bin
echo $PATH

# 如需要添加到 PATH
export PATH="/usr/local/bin:$PATH"
```

## 配置问题

### 问题: "Provider 'xxx' not found in config"

**原因**: Provider 未在 `~/.config/gcop/config.toml` 中配置

**解决方案**:
```bash
# 检查配置文件
cat ~/.config/gcop/config.toml

# 复制示例配置
cp examples/config.toml.example ~/.config/gcop/config.toml

# 编辑并添加 provider
vim ~/.config/gcop/config.toml
```

### 问题: "API key not found"

**原因**: 配置文件和环境变量中都没有 API key

**解决方案**:

**选项 1**: 添加到配置文件
```toml
[llm.providers.claude]
api_key = "sk-ant-your-key"
```

**选项 2**: 使用环境变量
```bash
export ANTHROPIC_API_KEY="sk-ant-your-key"
```

### 问题: "Unsupported api_style"

**原因**: 配置中的 `api_style` 值无效

**解决方案**: 使用支持的值之一：
- `"claude"` - 用于 Anthropic API 兼容服务
- `"openai"` - 用于 OpenAI API 兼容服务
- `"ollama"` - 用于本地 Ollama

## API 问题

### 问题: "401 Unauthorized"

**原因**: API key 无效或已过期

**解决方案**:
1. 验证 API key 是否正确
2. 检查 key 是否过期
3. 从 provider 控制台重新生成 key
4. 更新 config.toml 中的新 key

### 问题: "429 Rate Limit Exceeded"

**原因**: 请求过多

**解决方案**:
1. 稍等片刻再重试
2. 升级你的 API 计划
3. 临时切换到其他 provider

### 问题: "500 Internal Server Error"

**原因**: API 服务暂时不可用

**解决方案**:
1. 等待并重试
2. 检查 provider 的状态页面
3. 尝试其他 provider

### 问题: "Failed to parse Claude/OpenAI response"

**原因**: API 响应格式异常

**解决方案**:
```bash
# 使用详细模式查看原始响应
gcop -v commit

# 在调试输出中查找
# 查找 "Claude API response body:" 或 "OpenAI API response body:"
```

## 代码审查问题

### 问题: "Failed to parse review result"

**原因**: LLM 没有返回有效的 JSON

**解决方案**:

1. **使用详细模式**查看原始响应：
   ```bash
   gcop -v review changes
   ```

2. **检查自定义 prompt**（如果使用）：
   - 确保明确要求 JSON 格式
   - 提供准确的 JSON schema 示例

3. **尝试不同模型**：
   ```bash
   # 某些模型处理 JSON 更好
   gcop --provider openai review changes
   ```

4. **调整 temperature**：
   ```toml
   temperature = 0.1  # 更低 = 更一致的输出
   ```

## Git 问题

### 问题: "No staged changes found"

**原因**: Git 暂存区为空

**解决方案**:
```bash
# 先暂存变更
git add <files>

# 或暂存所有变更
git add .

# 然后运行 gcop
gcop commit
```

### 问题: "Not a git repository"

**原因**: 当前目录不是 git 仓库

**解决方案**:
```bash
# 初始化 git 仓库
git init

# 或在 git 仓库中运行 gcop
cd /path/to/your/git/repo
```

## 调试模式

对于任何问题，启用详细模式获取详细信息：

```bash
gcop -v commit
gcop -v review changes
```

这会显示：
- 配置加载过程
- API 请求和响应
- 发送给 LLM 的 prompt
- 响应解析过程

## 获取帮助

如果遇到这里未列出的问题：

1. 使用 `--verbose` 运行并检查日志
2. 查看[配置参考](configuration.md)
3. 查看 [Provider 设置指南](providers.md)
4. 在 GitHub 上开 issue，包括：
   - 你的配置文件（删除 API keys！）
   - 运行的命令
   - 错误信息
   - `gcop -v` 的输出（删除敏感信息）

## 参考

- [配置参考](configuration.md)
- [Provider 设置](providers.md)
- [自定义 Prompt](prompts.md)
