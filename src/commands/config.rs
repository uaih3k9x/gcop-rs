use crate::config::{self, load_config};
use crate::error::{GcopError, Result};
use crate::llm::provider::create_provider;
use crate::ui;
use std::process::Command;

pub async fn run(action: Option<crate::cli::ConfigAction>, colored: bool) -> Result<()> {
    // 默认行为：调用 edit
    let action = action.unwrap_or(crate::cli::ConfigAction::Edit);

    match action {
        crate::cli::ConfigAction::Edit => edit(colored),
        crate::cli::ConfigAction::Validate => validate(colored).await,
    }
}

/// 打开编辑器编辑配置文件
fn edit(colored: bool) -> Result<()> {
    let config_dir = config::get_config_dir()
        .ok_or_else(|| GcopError::Config("Failed to determine config directory".to_string()))?;

    let config_file = config_dir.join("config.toml");

    // 如果配置文件不存在，提示运行 init
    if !config_file.exists() {
        ui::error("Config file not found", colored);
        println!();
        println!("Run 'gcop-rs init' to create it, or create manually:");
        println!("  mkdir -p {}", config_dir.display());
        println!(
            "  cp examples/config.toml.example {}",
            config_file.display()
        );
        return Err(GcopError::Config("Config file not found".to_string()));
    }

    // 获取编辑器
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "vim".to_string());

    // 打开编辑器
    println!(
        "{}",
        ui::info(&format!("Opening {} ...", config_file.display()), colored)
    );

    let status = Command::new(&editor).arg(&config_file).status()?;

    if !status.success() {
        return Err(GcopError::Other("Editor exited with error".to_string()));
    }

    ui::success("Config file updated", colored);

    Ok(())
}

/// 验证配置
async fn validate(colored: bool) -> Result<()> {
    ui::step("1/2", "Loading configuration...", colored);

    // 加载配置
    let config = load_config()?;

    ui::success("Configuration loaded successfully", colored);
    println!();

    // 显示配置的 providers
    println!("Configured providers:");
    for name in config.llm.providers.keys() {
        println!("  • {}", name);
    }
    println!();

    // 测试默认 provider 连接
    ui::step("2/2", "Testing default provider connection...", colored);

    let provider = create_provider(&config, None)?;
    provider.validate().await?;

    ui::success(
        &format!(
            "Provider '{}' validated successfully",
            config.llm.default_provider
        ),
        colored,
    );

    Ok(())
}
