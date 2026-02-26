use php_guard_core::{encrypt_content, is_encrypted};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("用法: {} <php文件路径>", args[0]);
        println!("\n示例:");
        println!("  {} example.php", args[0]);
        std::process::exit(1);
    }

    let filepath = &args[1];

    match std::fs::read(filepath) {
        Ok(content) => {
            if is_encrypted(&content) {
                println!("文件已经是加密格式: {}", filepath);
                return;
            }

            let encrypted = encrypt_content(&content);

            let output_path = format!("{}.encrypted", filepath);
            match std::fs::write(&output_path, &encrypted) {
                Ok(_) => {
                    println!("加密成功!");
                    println!("原始文件: {}", filepath);
                    println!("加密文件: {}", output_path);
                    println!("原始大小: {} 字节", content.len());
                    println!("加密大小: {} 字节", encrypted.len());
                }
                Err(e) => {
                    eprintln!("写入文件失败: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("读取文件失败: {}", e);
            std::process::exit(1);
        }
    }
}
