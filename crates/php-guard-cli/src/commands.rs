use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

use php_guard_core::{encrypt_content, is_encrypted, read_and_decrypt_file};

pub fn encrypt(paths: &[String], output_dir: Option<&str>) -> Result<()> {
    println!("{}", "PHP-Guard 文件加密".green().bold());
    println!("{}", "=".repeat(40));

    let mut total = 0;
    let mut skipped = 0;

    for path in paths {
        let path_obj = Path::new(path);
        if path_obj.is_file() {
            match encrypt_single_file(&path_obj, output_dir)? {
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
                match encrypt_single_file(entry.path(), output_dir)? {
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

fn encrypt_single_file(path: &Path, output_dir: Option<&str>) -> Result<bool> {
    let content = fs::read(path)?;

    if is_encrypted(&content) {
        println!("{} 已加密，跳过: {}", "-".yellow(), path.display());
        return Ok(false);
    }

    // 创建备份文件
    let backup_path = path.with_extension("php.bak");
    if backup_path.exists() {
        println!("{} 备份文件已存在: {}", "-".yellow(), backup_path.display());
    } else {
        fs::copy(path, &backup_path)?;
        println!("{} 已创建备份: {}", "✓".green(), backup_path.display());
    }

    let encrypted = encrypt_content(&content);

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

pub fn check(paths: &[String]) -> Result<()> {
    println!("{}", "PHP-Guard 加密检查".green().bold());
    println!("{}", "=".repeat(40));

    let mut encrypted_count = 0;
    let mut total = 0;

    for path in paths {
        let path_obj = Path::new(path);
        if path_obj.is_file() {
            total += 1;
            if check_single_file(&path_obj)? {
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
                if check_single_file(entry.path())? {
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

fn check_single_file(path: &Path) -> Result<bool> {
    let content = fs::read(path)?;
    let is_enc = is_encrypted(&content);
    let status = if is_enc {
        format!("{} 已加密", "✓".green())
    } else {
        format!("{} 未加密", "✗".red())
    };
    println!("{}: {}", path.display(), status);
    Ok(is_enc)
}

pub fn decrypt(paths: &[String], output_dir: Option<&str>) -> Result<()> {
    println!("{}", "PHP-Guard 文件解密".green().bold());
    println!("{}", "=".repeat(40));

    let mut total = 0;
    let mut skipped = 0;

    for path in paths {
        let path_obj = Path::new(path);
        if path_obj.is_file() {
            match decrypt_single_file(&path_obj, output_dir)? {
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
                match decrypt_single_file(entry.path(), output_dir)? {
                    true => total += 1,
                    false => skipped += 1,
                }
            }
        }
    }

    println!("\n{} 解密完成: {} 个文件", "✓".green(), total);
    if skipped > 0 {
        println!("{} 跳过: {} 个文件 (未加密)", "-".yellow(), skipped);
    }

    Ok(())
}

fn decrypt_single_file(path: &Path, output_dir: Option<&str>) -> Result<bool> {
    let content = fs::read(path)?;

    if !is_encrypted(&content) {
        println!("{} 未加密，跳过: {}", "-".yellow(), path.display());
        return Ok(false);
    }

    let decrypted = read_and_decrypt_file(path)
        .map_err(|e| anyhow::anyhow!("解密失败: {}: {}", path.display(), e))?;

    let output_path = match output_dir {
        Some(dir) => {
            fs::create_dir_all(dir)?;
            Path::new(dir).join(path.file_name().unwrap())
        }
        None => path.to_path_buf(),
    };

    fs::write(&output_path, decrypted)?;
    println!("{} 解密成功: {}", "✓".green(), output_path.display());

    Ok(true)
}
