# Troubleshooting

Common issues and solutions for gcop-rs.

## Installation Issues

### Issue: `cargo build` fails

**Solution**:
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Issue: Binary not found after install

**Solution**:
```bash
# Check if binary exists
ls -la /usr/local/bin/gcop

# Verify PATH includes /usr/local/bin
echo $PATH

# Add to PATH if needed
export PATH="/usr/local/bin:$PATH"
```

## Configuration Issues

### Issue: "Provider 'xxx' not found in config"

**Cause**: Provider not configured in `~/.config/gcop/config.toml`

**Solution**:
```bash
# Check your config file
cat ~/.config/gcop/config.toml

# Copy example config
cp examples/config.toml.example ~/.config/gcop/config.toml

# Edit and add your provider
vim ~/.config/gcop/config.toml
```

### Issue: "API key not found"

**Cause**: No API key in config file or environment

**Solution**:

**Option 1**: Add to config file
```toml
[llm.providers.claude]
api_key = "sk-ant-your-key"
```

**Option 2**: Use environment variable
```bash
export ANTHROPIC_API_KEY="sk-ant-your-key"
```

### Issue: "Unsupported api_style"

**Cause**: Invalid `api_style` value in config

**Solution**: Use one of the supported values:
- `"claude"` - For Anthropic API compatible services
- `"openai"` - For OpenAI API compatible services
- `"ollama"` - For local Ollama

## API Issues

### Issue: "401 Unauthorized"

**Cause**: Invalid or expired API key

**Solution**:
1. Verify your API key is correct
2. Check if the key has expired
3. Regenerate key from provider's dashboard
4. Update config.toml with new key

### Issue: "429 Rate Limit Exceeded"

**Cause**: Too many requests

**Solution**:
1. Wait a few moments before retry
2. Upgrade your API plan
3. Switch to a different provider temporarily

### Issue: "500 Internal Server Error"

**Cause**: API service temporarily unavailable

**Solution**:
1. Wait and retry
2. Check provider's status page
3. Try a different provider

### Issue: "Failed to parse Claude/OpenAI response"

**Cause**: Unexpected API response format

**Solution**:
```bash
# Use verbose mode to see raw response
gcop -v commit

# Check the response in debug output
# Look for "Claude API response body:" or "OpenAI API response body:"
```

## Code Review Issues

### Issue: "Failed to parse review result"

**Cause**: LLM didn't return valid JSON

**Solution**:

1. **Use verbose mode** to see raw response:
   ```bash
   gcop -v review changes
   ```

2. **Check your custom prompt** (if using one):
   - Ensure it explicitly requests JSON format
   - Provide exact JSON schema example

3. **Try different model**:
   ```bash
   # Some models handle JSON better
   gcop --provider openai review changes
   ```

4. **Adjust temperature**:
   ```toml
   temperature = 0.1  # Lower = more consistent output
   ```

## Git Issues

### Issue: "No staged changes found"

**Cause**: Nothing added to git staging area

**Solution**:
```bash
# Stage your changes first
git add <files>

# Or stage all changes
git add .

# Then run gcop
gcop commit
```

### Issue: "Not a git repository"

**Cause**: Current directory is not a git repo

**Solution**:
```bash
# Initialize git repository
git init

# Or run gcop from within a git repository
cd /path/to/your/git/repo
```

## Debug Mode

For any issue, enable verbose mode to get detailed information:

```bash
gcop -v commit
gcop -v review changes
```

This shows:
- Configuration loading
- API requests and responses
- Prompt sent to LLM
- Response parsing

## Getting Help

If you encounter an issue not listed here:

1. Run with `--verbose` and check the logs
2. Check the [Configuration Reference](configuration.md)
3. Review the [Provider Setup Guide](providers.md)
4. Open an issue on GitHub with:
   - Your config file (remove API keys!)
   - Command you ran
   - Error message
   - Output from `gcop -v` (remove sensitive info)

## See Also

- [Configuration Reference](configuration.md)
- [Provider Setup](providers.md)
- [Custom Prompts](prompts.md)
