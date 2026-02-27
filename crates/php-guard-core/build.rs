use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config_generated.rs");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_dir = Path::new(&manifest_dir)
        .parent()
        .and_then(|p| p.parent())
        .expect("Failed to find workspace directory");

    let config_dir = env::var("PHP_GUARD_CONFIG_DIR").unwrap_or_else(|_| {
        workspace_dir
            .join(".php-guard")
            .to_string_lossy()
            .into_owned()
    });
    let config_file = Path::new(&config_dir).join("config.env");

    let (key, header) = if config_file.exists() {
        read_config_from_file(&config_file)
    } else {
        let key = generate_random_bytes(32);
        let header = generate_random_bytes(16);
        save_config_to_file(&config_file, &key, &header);
        (key, header)
    };

    let code = generate_config_code(&key, &header);
    fs::write(&dest_path, code).unwrap();

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

    (
        key.expect("PHP_GUARD_KEY not found"),
        header.expect("PHP_GUARD_HEADER not found"),
    )
}

fn save_config_to_file(path: &Path, key: &[u8], header: &[u8]) {
    let config_dir = path.parent().unwrap();
    fs::create_dir_all(config_dir).ok();

    let content = format!(
        r#"# PHP-Guard 配置文件
# 加密密钥 (256位)
export PHP_GUARD_KEY="{}"

# 文件头部标识 (128位)
export PHP_GUARD_HEADER="{}"
"#,
        bytes_to_hex(key),
        bytes_to_hex(header)
    );

    fs::write(path, content).expect("Failed to write config file");
}

fn generate_random_bytes(len: usize) -> Vec<u8> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let mut seed = timestamp as u64;
    (0..len)
        .map(|_| {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            ((seed >> 16) & 0xFF) as u8
        })
        .collect()
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
    format!(
        "pub const KEY: &[u8] = &[{}];\npub const HEADER: &[u8] = &[{}];\n",
        format_bytes_for_rust(key),
        format_bytes_for_rust(header)
    )
}

fn format_bytes_for_rust(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("0x{:02x}", b))
        .collect::<Vec<_>>()
        .join(", ")
}
