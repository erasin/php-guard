# PHP-Guard 项目开发指南

本文档为 AI 编程助手提供项目开发规范和常用命令参考。

## 项目概述

PHP-Guard 是一个高性能、跨平台的 PHP7+ 代码加密扩展，使用 Rust (phper 框架) 开发。采用 Workspace 结构组织多个 crate。

## 项目结构

```
php-guard/
├── crates/
│   ├── php-guard-core/    # 核心库 (加密/解密算法)
│   ├── php-guard-ext/     # PHP 扩展 (cdylib)
│   └── php-guard-cli/     # CLI 工具
├── .php-guard/            # 配置目录 (密钥)
├── test/                  # 测试 PHP 文件
└── documents/             # 项目文档
```

## 构建命令

### 开发构建

```bash
# 构建所有组件
cargo build

# 构建特定 crate
cargo build -p php-guard-core
cargo build -p php-guard-cli
cargo build -p php-guard-ext

# 发布构建
cargo build --release

# 指定 PHP 版本构建扩展
cargo build -p php-guard-ext --release
# 或使用 Makefile
make build PHP_CONFIG=/opt/remi/php82/root/usr/bin/php-config
```

### 测试命令

```bash
# 运行所有测试
cargo test

# 运行特定 crate 的测试
cargo test -p php-guard-core

# 运行单个测试 (按名称匹配)
cargo test -p php-guard-core test_encode_decode

# 运行单个测试文件中的测试
cargo test -p php-guard-core --test encrypt_file

# 显示测试输出
cargo test -- --nocapture

# 运行特定模块的测试
cargo test -p php-guard-core crypto::tests
```

### 代码检查

```bash
# Clippy 检查
cargo clippy

# 严格检查 (警告视为错误)
cargo clippy -- -D warnings

# 格式检查
cargo fmt -- --check

# 自动格式化
cargo fmt

# 完整 lint (Makefile)
make lint
```

### 安装

```bash
# 使用 Makefile 安装
make install

# 或手动安装
sudo cp target/release/libphp_guard_ext.so $(php-config --extension-dir)/php_guard.so
cp target/release/php-guard-cli ~/.cargo/bin/
```

## 代码风格规范

### 导入顺序

按以下顺序组织导入，每组之间空一行：

```rust
// 1. 标准库
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

// 2. 外部 crate
use anyhow::Result;
use phper::modules::Module;

// 3. 内部 crate (当前 crate)
use crate::config::HEADER;
use crate::crypto::{decode, is_encrypted};

// 4. 父模块
use super::hooks;
```

### 命名规范

| 类型 | 风格 | 示例 |
|------|------|------|
| 函数/变量 | snake_case | `encrypt_file`, `file_size` |
| 类型/Trait | PascalCase | `FileHandler`, `CryptoError` |
| 常量 | SCREAMING_SNAKE_CASE | `HEADER`, `KEY` |
| 模块 | snake_case | `file_handler`, `php_extension` |
| Cargo crate | kebab-case | `php-guard-core` |

### 函数定义

```rust
// 公开函数使用 pub，参数使用泛型约束
pub fn read_and_decrypt_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    // 实现
}

// 内部辅助函数使用 fn
fn encrypt_single_file(path: &Path, output_dir: Option<&str>) -> Result<bool> {
    // 实现
}

// FFI 函数使用 extern "C" 和 #[unsafe(no_mangle)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn php_guard_compile_file(
    file_handle: *mut zend_file_handle,
    type_: c_int,
) -> *mut sys::_zend_op_array {
    // 实现
}
```

### 错误处理

```rust
// CLI 工具使用 anyhow::Result
use anyhow::Result;

pub fn encrypt(paths: &[String], output_dir: Option<&str>) -> Result<()> {
    let content = fs::read(path)?;
    // ...
    Ok(())
}

// 核心库使用 std::io::Result 或自定义错误类型
pub fn read_and_decrypt_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    // 使用 ? 传播错误
    let mut file = File::open(path)?;
    
    // 显式构造错误
    return Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "File is not encrypted or has wrong header",
    ));
}

// PHP 扩展使用 phper::Result
fn php_guard_encode(arguments: &mut [ZVal]) -> phper::Result<Option<ZString>> {
    let content = arguments[0].expect_z_str()?;
    Ok(Some(ZString::new(&encrypted)))
}
```

### 模块导出

```rust
// lib.rs 中重新导出常用项
pub mod config;
pub mod crypto;
pub mod file_handler;

pub use config::{HEADER, KEY};
pub use crypto::{decode, encode, is_encrypted};
pub use file_handler::{
    check_file_encrypted, encrypt_file, read_and_decrypt_file,
};
```

### 条件编译

```rust
// PHP 版本条件编译
#[cfg(not(phper_major_version = "8"))]
use std::ffi::CStr;

#[cfg(phper_major_version = "8")]
use phper::sys::{self, zend_compile_file, zend_file_handle};

// 常量定义
#[cfg(not(phper_major_version = "8"))]
const ZEND_HANDLE_FP: zend_stream_type = zend_stream_type_ZEND_HANDLE_FP;

#[cfg(phper_major_version = "8")]
const ZEND_HANDLE_FP: u8 = 1;
```

### 测试编写

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let original = b"<?php echo 'Hello, World!'; ?>".to_vec();
        let mut data = original.clone();

        encode(&mut data);
        assert!(!is_encrypted(&data));
        assert_ne!(data, original);

        decode(&mut data);
        assert_eq!(data, original);
    }

    #[test]
    fn test_is_encrypted() {
        let encrypted_with_header = [HEADER, b"test"].concat();
        assert!(is_encrypted(&encrypted_with_header));

        let plain = b"<?php echo 'test';";
        assert!(!is_encrypted(plain));
    }
}
```

### unsafe 代码规范

```rust
// unsafe 块必须注释说明安全性
unsafe fn call_original(
    file_handle: *mut zend_file_handle,
    type_: c_int,
) -> *mut sys::_zend_op_array {
    // SAFETY: ORIGINAL_COMPILE_FILE 在 init_hooks 中已正确初始化
    unsafe {
        match ORIGINAL_COMPILE_FILE {
            Some(original) => original(file_handle, type_),
            None => ptr::null_mut(),
        }
    }
}

// 允许 unsafe 在合理位置
#[allow(unused_unsafe)]
pub unsafe extern "C" fn php_guard_compile_file(...) { }
```

## Crate 职责

### php-guard-core
- 加密/解密算法实现
- 文件处理工具
- 配置管理
- 纯 Rust 逻辑，无 PHP 依赖

### php-guard-ext
- PHP 扩展入口
- zend_compile_file hook
- PHP 函数导出
- 依赖 phper 框架

### php-guard-cli
- 命令行工具
- 文件加密/解密/检查
- 用户交互

## 依赖库

| Crate | 用途 |
|-------|------|
| phper | PHP 扩展框架 |
| clap | CLI 参数解析 |
| anyhow | 错误处理 (CLI) |
| colored | 终端着色 |
| walkdir | 目录遍历 |
| tempfile | 临时文件 |

## 配置说明

首次构建需要生成密钥配置：

```bash
# 生成配置
./scripts/generate-key.sh

# 加载配置
source .php-guard/config.env

# 配置文件位置
.php-guard/config.env
```

配置文件包含：
- `PHP_GUARD_KEY`: 256位加密密钥
- `PHP_GUARD_HEADER`: 128位文件头部标识

## 注意事项

1. **不要提交 `.php-guard/config.env` 到版本控制**
2. **修改核心库后需重新构建扩展**
3. **PHP 扩展测试需要实际 PHP 环境**
4. **使用 `std::mem::forget` 管理 FFI 资源所有权**
5. **跨平台代码注意条件编译**
