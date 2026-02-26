use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config_generated.rs");

    // 配置文件路径
    let config_dir = env::var("PHP_GUARD_CONFIG_DIR").unwrap_or_else(|_| ".php-guard".to_string());
    let config_file = Path::new(&config_dir).join("config.env");

    // 尝试从配置文件读取
    let (key, header) = if config_file.exists() {
        println!(
            "cargo:warning=Reading configuration from: {}",
            config_file.display()
        );
        read_config_from_file(&config_file)
    } else {
        // 生成默认配置
        println!("cargo:warning=Generating default configuration");
        let key = generate_random_bytes(32);
        let header = generate_random_bytes(16);

        // 保存到配置文件
        save_config_to_file(&config_file, &key, &header);

        (key, header)
    };

    // 生成 Rust 代码
    let code = generate_config_code(&key, &header);

    fs::write(&dest_path, code).unwrap();

    // 重新构建如果配置文件改变
    println!("cargo:rerun-if-changed={}", config_file.display());
    println!("cargo:rerun-if-env-changed=PHP_GUARD_CONFIG_DIR");
}

fn read_config_from_file(path: &Path) -> (Vec<u8>, Vec<u8>) {
    let content = fs::read_to_string(path).expect("Failed to read config file");

    let mut key = None;
    let mut header = None;

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with("export PHP_GUARD_KEY=") || line.starts_with("PHP_GUARD_KEY=") {
            let value = line.split('=').nth(1).unwrap().trim().trim_matches('"');
            key = Some(hex_to_bytes(value));
        } else if line.starts_with("export PHP_GUARD_HEADER=")
            || line.starts_with("PHP_GUARD_HEADER=")
            || line.starts_with("set PHP_GUARD_HEADER=")
        {
            let value = line.split('=').nth(1).unwrap().trim().trim_matches('"');
            header = Some(hex_to_bytes(value));
        }
    }

    let key = key.expect("PHP_GUARD_KEY not found in config file");
    let header = header.expect("PHP_GUARD_HEADER not found in config file");

    (key, header)
}

fn save_config_to_file(path: &Path, key: &[u8], header: &[u8]) {
    let config_dir = path.parent().unwrap();
    fs::create_dir_all(config_dir).ok();

    let key_hex = bytes_to_hex(key);
    let header_hex = bytes_to_hex(header);

    let content = format!(
        r#"# PHP-Guard 配置文件
# 由 build.rs 自动生成

# 加密密钥 (256位)
export PHP_GUARD_KEY="{}"

# 文件头部标识 (128位)
export PHP_GUARD_HEADER="{}"
"#,
        key_hex, header_hex
    );

    fs::write(path, content).expect("Failed to write config file");
}

fn generate_random_bytes(len: usize) -> Vec<u8> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut bytes = Vec::with_capacity(len);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    // 简单的伪随机生成器（实际使用时应该用更安全的方法）
    let mut seed = timestamp as u64;
    for _i in 0..len {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        bytes.push(((seed >> 16) & 0xFF) as u8);
    }

    bytes
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn generate_config_code(key: &[u8], header: &[u8]) -> String {
    let key_bytes = format_bytes_for_rust(key);
    let header_bytes = format_bytes_for_rust(header);

    format!(
        r#"pub const KEY: &[u8] = &[{}];
pub const HEADER: &[u8] = &[{}];

#[cfg(feature = "php-extension")]
pub const MODULE_NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(feature = "php-extension")]
pub const MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(feature = "php-extension")]
pub const MODULE_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
"#,
        key_bytes, header_bytes
    )
}

fn format_bytes_for_rust(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("0x{:02x}", b))
        .collect::<Vec<_>>()
        .join(", ")
}
