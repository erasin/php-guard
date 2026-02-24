use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::config::GuardConfig;

pub fn generate_key(header_len: usize, key_len: usize, output: Option<String>) -> Result<()> {
    println!("{}", "PHP-Guard 密钥生成".green().bold());
    println!("{}", "=".repeat(40));
    println!("加密头长度: {} bytes", header_len);
    println!("密钥长度: {} bytes\n", key_len);

    let config = GuardConfig::generate(header_len, key_len);

    if let Some(path) = output {
        config.save(&path)?;
        println!("{} 配置已保存到: {}", "✓".green(), path);
    }

    println!("\n{}", "=== Rust (src/config.rs) ===".cyan());
    print!("{}", config.to_rust_code());

    println!("{}", "=== PHP (tools/php-guard.php) ===".cyan());
    print!("{}", config.to_php_code());

    println!("\n{} 请确保两个文件使用相同的配置!", "⚠".yellow());

    Ok(())
}

pub fn encrypt(paths: &[String], output: Option<&str>, config_path: &str) -> Result<()> {
    let config = GuardConfig::load(config_path)?;

    println!("{}", "PHP-Guard 文件加密".green().bold());
    println!("{}", "=".repeat(40));
    println!("配置文件: {}", config_path);

    let mut total = 0;
    let mut skipped = 0;

    for path in paths {
        let path_obj = Path::new(path);
        if path_obj.is_file() {
            match encrypt_single_file(path_obj, &config, output)? {
                true => total += 1,
                false => skipped += 1,
            }
        } else if path_obj.is_dir() {
            for entry in walkdir::WalkDir::new(path_obj)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .map(|ext| ext == "php")
                        .unwrap_or(false)
                })
            {
                match encrypt_single_file(entry.path(), &config, output)? {
                    true => total += 1,
                    false => skipped += 1,
                }
            }
        }
    }

    println!("\n{} 加密完成: {} 个文件", "✓".green(), total);
    if skipped > 0 {
        println!("{} 跳过: {} 个文件 (已加密)", "-".yellow(), skipped);
    }

    Ok(())
}

fn encrypt_single_file(
    path: &Path,
    config: &GuardConfig,
    output_dir: Option<&str>,
) -> Result<bool> {
    let content = fs::read(path)?;

    if content.len() >= config.header.len() && &content[..config.header.len()] == config.header {
        println!("{} 已加密，跳过: {}", "-".yellow(), path.display());
        return Ok(false);
    }

    let encrypted = encrypt_content_with_config(&content, config);

    let output_path = match output_dir {
        Some(dir) => {
            fs::create_dir_all(dir)?;
            Path::new(dir).join(path.file_name().unwrap())
        }
        None => path.to_path_buf(),
    };

    fs::write(&output_path, &encrypted)?;
    println!("{} 加密成功: {}", "✓".green(), output_path.display());

    Ok(true)
}

fn encrypt_content_with_config(content: &[u8], config: &GuardConfig) -> Vec<u8> {
    let mut result = Vec::with_capacity(config.header.len() + content.len());
    result.extend_from_slice(&config.header);

    let mut data = content.to_vec();
    let key = &config.key;
    let key_len = key.len();
    let mut p: usize = 0;

    for (i, byte) in data.iter_mut().enumerate() {
        if i & 1 == 1 {
            p = p.wrapping_add(key[p] as usize).wrapping_add(i);
            p %= key_len;
            let t = key[p];
            *byte = !(*byte ^ t);
        }
    }

    result.extend_from_slice(&data);
    result
}

pub fn check(paths: &[String], config_path: &str) -> Result<()> {
    let config = GuardConfig::load(config_path)?;

    println!("{}", "PHP-Guard 加密检查".green().bold());
    println!("{}", "=".repeat(40));

    let mut encrypted_count = 0;
    let mut total = 0;

    for path in paths {
        let path_obj = Path::new(path);
        if path_obj.is_file() {
            total += 1;
            if check_single_file(path_obj, &config)? {
                encrypted_count += 1;
            }
        } else if path_obj.is_dir() {
            for entry in walkdir::WalkDir::new(path_obj)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .map(|ext| ext == "php")
                        .unwrap_or(false)
                })
            {
                total += 1;
                if check_single_file(entry.path(), &config)? {
                    encrypted_count += 1;
                }
            }
        }
    }

    println!(
        "\n{} 统计: {}/{} 个文件已加密",
        "✓".green(),
        encrypted_count,
        total
    );

    Ok(())
}

fn check_single_file(path: &Path, config: &GuardConfig) -> Result<bool> {
    let content = fs::read(path)?;

    let is_enc =
        content.len() >= config.header.len() && &content[..config.header.len()] == config.header;

    let status = if is_enc {
        format!("{} 已加密", "✓".green())
    } else {
        format!("{} 未加密", "✗".red())
    };

    println!("{}: {}", path.display(), status);
    Ok(is_enc)
}

pub fn verify(rust_config: &str, php_config: &str) -> Result<()> {
    println!("{}", "PHP-Guard 配置验证".green().bold());
    println!("{}", "=".repeat(40));
    println!("Rust 配置: {}", rust_config);
    println!("PHP 配置:  {}\n", php_config);

    let rust_content = fs::read_to_string(rust_config)
        .with_context(|| format!("无法读取 Rust 配置文件: {}", rust_config))?;
    let php_content = fs::read_to_string(php_config)
        .with_context(|| format!("无法读取 PHP 配置文件: {}", php_config))?;

    let rust_header = parse_rust_bytes(&rust_content, "HEADER")?;
    let rust_key = parse_rust_bytes(&rust_content, "KEY")?;
    let php_header = parse_php_bytes(&php_content, "HEADER")?;
    let php_key = parse_php_bytes(&php_content, "KEY")?;

    let mut has_error = false;

    println!("--- HEADER ---");
    println!(
        "Rust ({} bytes): {}...",
        rust_header.len(),
        format_bytes_preview(&rust_header)
    );
    println!(
        "PHP  ({} bytes): {}...",
        php_header.len(),
        format_bytes_preview(&php_header)
    );

    if rust_header == php_header {
        println!("{} HEADER 配置一致\n", "✓".green());
    } else {
        println!("{} HEADER 配置不一致!\n", "✗".red());
        has_error = true;
    }

    println!("--- KEY ---");
    println!(
        "Rust ({} bytes): {}...",
        rust_key.len(),
        format_bytes_preview(&rust_key)
    );
    println!(
        "PHP  ({} bytes): {}...",
        php_key.len(),
        format_bytes_preview(&php_key)
    );

    if rust_key == php_key {
        println!("{} KEY 配置一致\n", "✓".green());
    } else {
        println!("{} KEY 配置不一致!\n", "✗".red());
        has_error = true;
    }

    if has_error {
        anyhow::bail!("配置验证失败!");
    }

    println!("{} 配置验证通过!", "✓".green());
    Ok(())
}

fn parse_rust_bytes(content: &str, name: &str) -> Result<Vec<u8>> {
    let pattern = format!("pub const {}: &[u8] = &[", name);
    let start = content
        .find(&pattern)
        .with_context(|| format!("找不到 Rust {} 定义", name))?;
    let end = content[start..]
        .find("];")
        .with_context(|| format!("无法解析 Rust {}", name))?;
    let bytes_str = &content[start + pattern.len()..start + end];
    parse_hex_bytes(bytes_str)
}

fn parse_php_bytes(content: &str, name: &str) -> Result<Vec<u8>> {
    let pattern = format!("const {} = [", name);
    let start = content
        .find(&pattern)
        .with_context(|| format!("找不到 PHP {} 定义", name))?;
    let end = content[start..]
        .find("];")
        .with_context(|| format!("无法解析 PHP {}", name))?;
    let bytes_str = &content[start + pattern.len()..start + end];
    parse_hex_bytes(bytes_str)
}

fn parse_hex_bytes(s: &str) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    for part in s.split(',') {
        let part = part.trim();
        if part.starts_with("0x") || part.starts_with("0X") {
            let hex = &part[2..];
            if !hex.is_empty() {
                let b = u8::from_str_radix(hex, 16)
                    .with_context(|| format!("无法解析字节: {}", part))?;
                bytes.push(b);
            }
        }
    }
    Ok(bytes)
}

fn format_bytes_preview(bytes: &[u8]) -> String {
    bytes
        .iter()
        .take(8)
        .map(|b| format!("0x{:02X}", b))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn init(output: &str) -> Result<()> {
    println!("{}", "PHP-Guard 初始化".green().bold());
    println!("{}", "=".repeat(40));

    let config = GuardConfig::generate(12, 16);
    config.save(output)?;

    println!("{} 配置文件已创建: {}", "✓".green(), output);
    println!("\n下一步:");
    println!("1. 查看/修改配置: {}", output);
    println!("2. 生成密钥: php-guard generate-key");
    println!("3. 加密文件: php-guard encrypt <文件或目录>");

    Ok(())
}

pub fn build(release: bool, php_config_path: Option<&str>) -> Result<()> {
    println!("{}", "PHP-Guard 构建".green().bold());
    println!("{}", "=".repeat(40));

    if let Some(path) = php_config_path {
        std::env::set_var("PHP_CONFIG", path);
        println!("PHP 配置: {}", path);
    }

    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build");

    if release {
        cmd.arg("--release");
    }

    cmd.args(["--features", "php-extension"]);

    println!(
        "执行: cargo build {} --features php-extension",
        if release { "--release" } else { "" }
    );

    let status = cmd.status().context("无法执行 cargo build")?;

    if status.success() {
        let target_dir = if release { "release" } else { "debug" };
        println!("\n{} 构建成功!", "✓".green());
        println!("扩展文件: target/{}/libphp_guard.so", target_dir);
    } else {
        anyhow::bail!("构建失败!");
    }

    Ok(())
}
