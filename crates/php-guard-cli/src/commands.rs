use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

use php_guard::{is_encrypted, HEADER};

pub fn encrypt(paths: &[String], output_dir: Option<&str>) -> Result<()> {
    println!("{}", "PHP-Guard 文件加密".green().bold());
    println!("{}", "=".repeat(40));

    let mut total = 0;
    let mut skipped = 0;

    for path in paths {
        let path_obj = Path::new(path);
        if path_obj.is_file() {
            match encrypt_single_file(path_obj, output_dir)? {
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

    let encrypted = php_guard::encrypt_content(&content);

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
            if check_single_file(path_obj)? {
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
