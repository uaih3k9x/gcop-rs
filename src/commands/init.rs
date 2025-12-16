use crate::config;
use crate::error::{GcopError, Result};
use crate::ui;
use std::fs;
use std::process::Command;

pub fn run(action: Option<crate::cli::InitAction>, colored: bool) -> Result<()> {
    match action {
        None => run_config(false, colored),
        Some(crate::cli::InitAction::Config { force }) => run_config(force, colored),
        Some(crate::cli::InitAction::Alias { force }) => run_alias(force, colored),
    }
}

/// 初始化配置文件
pub fn run_config(force: bool, colored: bool) -> Result<()> {
    // 1. 获取配置目录和文件路径
    let config_dir = config::get_config_dir()
        .ok_or_else(|| GcopError::Config("Failed to determine config directory".to_string()))?;

    let config_file = config_dir.join("config.toml");

    // 2. 检查配置文件是否已存在
    if config_file.exists() && !force {
        ui::warning(
            &format!("Config file already exists: {}", config_file.display()),
            colored,
        );
        println!();
        println!("Use --force to overwrite, or edit it directly:");
        println!("  gcop-rs config edit");
        return Ok(());
    }

    // 3. 创建配置目录
    fs::create_dir_all(&config_dir)?;
    ui::success(
        &format!("Created config directory: {}", config_dir.display()),
        colored,
    );

    // 4. 复制示例配置
    let example_config = include_str!("../../examples/config.toml.example");
    fs::write(&config_file, example_config)?;
    ui::success(
        &format!("Created config file: {}", config_file.display()),
        colored,
    );

    // 5. 设置文件权限（仅 Unix）
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&config_file)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&config_file, perms)?;
        ui::success("Set file permissions: 600", colored);
    }

    // 6. 显示下一步提示
    println!();
    println!("{}", ui::info("Next steps:", colored));
    println!("  1. Edit the config file:");
    println!("     gcop-rs config edit");
    println!();
    println!("  2. Add your API key to [llm.providers.claude]");
    println!("     Get key from: https://console.anthropic.com/");
    println!();
    println!("  3. Test it:");
    println!("     gcop-rs commit");
    println!();
    println!("{}", ui::info("Optional - Add git alias:", colored));
    println!("  git config --global alias.cop '!gcop-rs'");
    println!("  Then use: git cop commit");
    println!();
    println!("See docs/configuration.md for more options.");

    Ok(())
}

/// 添加 git alias
pub fn run_alias(force: bool, colored: bool) -> Result<()> {
    // 1. 检测 gcop-rs 命令是否在 PATH 中
    if !is_gcop_in_path() {
        ui::error("'gcop-rs' command not found in PATH", colored);
        println!();
        println!("Install gcop-rs first:");
        println!("  sudo cp target/release/gcop-rs /usr/local/bin/gcop-rs");
        println!();
        println!("Or add to PATH:");
        println!("  export PATH=\"$HOME/.local/bin:$PATH\"");
        return Err(GcopError::Config("gcop-rs not in PATH".to_string()));
    }

    // 2. 检测现有 alias
    let existing_alias = get_git_alias("cop")?;

    if let Some(existing) = existing_alias {
        if existing == "!gcop-rs" {
            // 已存在且相同
            println!(
                "{}",
                ui::info(
                    "Git alias 'cop' already exists and points to: !gcop-rs",
                    colored
                )
            );
            println!("  No changes needed.");
            return Ok(());
        } else {
            // 已存在但不同
            if !force {
                ui::warning(
                    &format!("Git alias 'cop' already exists and points to: {}", existing),
                    colored,
                );
                println!();
                println!("Use --force to overwrite:");
                println!("  gcop-rs init alias --force");
                return Ok(());
            } else {
                ui::warning("Overwriting existing alias 'cop'", colored);
            }
        }
    }

    // 3. 添加 git alias
    let status = Command::new("git")
        .args(["config", "--global", "alias.cop", "!gcop-rs"])
        .status()?;

    if !status.success() {
        ui::error("Failed to add git alias", colored);
        return Err(GcopError::Other("git config command failed".to_string()));
    }

    // 4. 验证添加成功
    let verify = get_git_alias("cop")?;
    if verify != Some("!gcop-rs".to_string()) {
        ui::error("Failed to verify git alias", colored);
        return Err(GcopError::Other("Alias verification failed".to_string()));
    }

    // 5. 显示成功信息
    ui::success("Added git alias: git cop", colored);
    println!();
    println!("Now you can use:");
    println!("  git cop commit");
    println!("  git cop review changes");

    Ok(())
}

/// 检查 gcop-rs 命令是否在 PATH 中
fn is_gcop_in_path() -> bool {
    Command::new("which")
        .arg("gcop-rs")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// 获取 git alias 的值
fn get_git_alias(name: &str) -> Result<Option<String>> {
    let output = Command::new("git")
        .args(["config", "--global", &format!("alias.{}", name)])
        .output()?;

    if output.status.success() {
        let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(Some(value))
    } else {
        Ok(None)
    }
}
