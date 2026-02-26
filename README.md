# PHP-Guard

[![CI](https://github.com/yourname/php-guard/workflows/CI/badge.svg)](https://github.com/yourname/php-guard/actions)
[![Rust](https://img.shields.io/badge/Rust-1.85+-orange.svg)](https://www.rust-lang.org/)
[![PHP](https://img.shields.io/badge/PHP-7.0%20--%208.5-blue.svg)](https://php.net)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

一个简洁、高性能、跨平台的 PHP7+ 代码加密扩展，使用 Rust (phper 框架) 开发。

## 特性

- 🚀 **高性能** - 采用轻量级 XOR 算法，运行时性能损耗 < 1%
- 🔒 **透明解密** - 无需修改 PHP 代码，扩展自动处理加密文件
- 🌍 **跨平台** - 支持 Linux x64/ARM64、Windows x64
- 🔧 **兼容性好** - 兼容 OPcache、Xdebug 等扩展
- ⚙️ **安全配置** - 编译时生成密钥，确保安全性
- 📦 **CLI 工具** - 提供统一的命令行工具
- 🧩 **模块化设计** - 核心库、扩展、CLI 分离，便于复用

## 项目结构

```
php-guard/
├── Cargo.toml              # Workspace 配置
├── crates/
│   ├── php-guard-core/     # 核心库 (加密/解密算法)
│   │   ├── Cargo.toml
│   │   ├── build.rs        # 配置生成
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config.rs   # 配置 (自动生成)
│   │       ├── crypto.rs   # 加密算法
│   │       └── file_handler.rs
│   ├── php-guard-ext/      # PHP 扩展
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── hooks.rs    # PHP hook
│   │       └── php_extension.rs
│   └── php-guard-cli/      # CLI 工具
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           └── commands.rs
├── scripts/
│   ├── generate-key.sh     # 密钥生成 (Linux/macOS)
│   └── generate-key.bat    # 密钥生成 (Windows)
├── .php-guard/
│   └── config.env          # 密钥配置
└── .github/workflows/
    ├── ci.yml              # 持续集成
    └── release.yml         # 自动发布
```

## 安装

### 从源码编译

```bash
# 1. 克隆项目
git clone https://github.com/yourname/php-guard.git
cd php-guard

# 2. 生成加密密钥和配置
./scripts/generate-key.sh  # Linux/macOS
# 或
.\scripts\generate-key.bat  # Windows

# 3. 加载配置
source .php-guard/config.env  # Linux/macOS

# 4. 编译扩展
cargo build -p php-guard-ext --release

# 5. 安装
sudo cp target/release/libphp_guard_ext.so $(php-config --extension-dir)/php_guard.so
```

### Docker 构建

```bash
# 构建指定 PHP 版本
docker build --build-arg PHP_VERSION=8.3 -t php-guard .

# 提取编译产物
docker run --rm -v $(pwd)/dist:/dist php-guard cp /build/target/release/libphp_guard_ext.so /dist/
```

## 快速开始

### 1. 生成配置

首次使用需要生成加密密钥和头部标识：

**Linux/macOS:**
```bash
./scripts/generate-key.sh
source .php-guard/config.env
```

**Windows:**
```cmd
.\scripts\generate-key.bat
.\.php-guard\config.env
```

### 2. 构建 PHP 扩展

```bash
cargo build -p php-guard-ext --release
```

### 3. 安装扩展

```bash
sudo cp target/release/libphp_guard_ext.so $(php-config --extension-dir)/php_guard.so
echo "extension=php_guard.so" | sudo tee /etc/php/conf.d/php_guard.ini
```

### 4. 加密 PHP 文件

使用 CLI 工具加密文件：

```bash
# 构建CLI工具
cargo build -p php-guard-cli --release

# 加密单个文件
./target/release/php-guard-cli encrypt src/file.php

# 加密目录
./target/release/php-guard-cli encrypt src/

# 检查加密状态
./target/release/php-guard-cli check src/
```

## 工作原理

1. **编译时配置**: 使用 `scripts/generate-key.sh` 生成密钥和头部标识
2. **构建集成**: `build.rs` 在编译时读取配置并生成 Rust 代码
3. **透明加密**: CLI 工具使用相同的密钥加密 PHP 文件
4. **自动解密**: PHP 扩展 hook 编译过程，自动解密加密文件

## 配置说明

配置文件位于 `.php-guard/config.env`，包含：

- `PHP_GUARD_KEY`: 256位加密密钥 (64个十六进制字符)
- `PHP_GUARD_HEADER`: 128位文件头部标识 (32个十六进制字符)

**重要提示:**
- 请妥善保管配置文件
- 不要将配置文件提交到版本控制系统
- 不同环境应使用不同的配置

## PHP API

```php
// 加密内容
$encrypted = php_guard_encode($content);

// 检查是否已加密
if (php_guard_is_encrypted($content)) {
    // ...
}

// 获取版本
echo php_guard_version(); // "0.1.0"
```

## 兼容性

| 类别 | 项目 | 状态 |
|------|------|------|
| OS | Linux | ✅ (x64, ARM64) |
| OS | macOS | ✅ (x64, Apple Silicon) |
| OS | Windows | ⚠️ (通过 WSL 或 Docker) |
| PHP | 7.0 - 7.4 | ✅ |
| PHP | 8.0 - 8.5 | ✅ |
| SAPI | CLI | ✅ |
| SAPI | FPM | ✅ |
| 扩展 | OPcache | ✅ |
| 扩展 | Xdebug | ✅ |

## 安全最佳实践

1. **密钥管理**
   - 使用强随机密钥
   - 定期更换密钥
   - 不同环境使用不同密钥

2. **文件保护**
   - 备份原始文件
   - 不要提交加密文件到版本控制
   - 限制配置文件访问权限

3. **选择性加密**
   - 只加密核心业务代码
   - 避免加密框架和库文件

## 开发

```bash
# 运行所有测试
cargo test

# 测试核心库
cargo test -p php-guard-core

# 构建 CLI
cargo build -p php-guard-cli --release

# 构建扩展
cargo build -p php-guard-ext --release

# 构建所有组件
cargo build --release

# 代码检查
cargo clippy
cargo fmt --check
```

## 致谢

- [tonyenc](https://github.com/lihancong/tonyenc) - 原始 C 实现
- [phper](https://github.com/phper-framework/phper) - Rust PHP 扩展框架

## 许可证

MIT License
