# PHP-Guard

[![CI](https://github.com/yourname/php-guard/workflows/CI/badge.svg)](https://github.com/yourname/php-guard/actions)
[![Rust](https://img.shields.io/badge/Rust-1.85+-orange.svg)](https://www.rust-lang.org/)
[![PHP](https://img.shields.io/badge/PHP-7.0%20--%208.5-blue.svg)](https://php.net)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

一个简洁、高性能、跨平台的 PHP7+ 代码加密扩展，使用 Rust (phper 框架) 开发。

## 特性

- 🚀 **高性能** - 采用轻量级 XOR 算法，运行时性能损耗 < 1%
- 🔒 **透明解密** - 无需修改 PHP 代码，扩展自动处理加密文件
- 🌍 **跨平台** - 支持 Linux、macOS
- 🔧 **兼容性好** - 兼容 OPcache、Xdebug 等扩展
- ⚙️ **可定制** - 支持自定义加密头和密钥
- 📦 **CLI 工具** - 提供统一的命令行工具

## 安装

### 下载预编译版本

从 [Releases](https://github.com/yourname/php-guard/releases) 页面下载：

**CLI 工具:**
- `php-guard-linux-x64.tar.gz` - Linux x64
- `php-guard-linux-aarch64.tar.gz` - Linux ARM64
- `php-guard-macos-x64.tar.gz` - macOS Intel
- `php-guard-macos-arm64.tar.gz` - macOS Apple Silicon
- `php-guard-windows-x64.zip` - Windows x64

**PHP 扩展:**
- `php_guard-php8.3-linux-x64.tar.gz` - Linux PHP 8.3 (x64)
- `php_guard-php8.3-linux-aarch64.tar.gz` - Linux PHP 8.3 (ARM64)
- `php_guard-php8.2-linux-x64.tar.gz` - Linux PHP 8.2 (x64)
- `php_guard-php8.2-linux-aarch64.tar.gz` - Linux PHP 8.2 (ARM64)
- 等等...

### 从源码编译

```bash
# 1. 克隆项目
git clone https://github.com/yourname/php-guard.git
cd php-guard

# 2. 使用 CLI 工具生成密钥
cargo run -p php-guard-cli -- generate-key

# 3. 编译扩展
cargo build --features php-extension --release

# 4. 安装
sudo cp target/release/libphp_guard.so $(php-config --extension-dir)/php_guard.so
```

### Docker 构建

```bash
# 构建指定 PHP 版本
docker build --build-arg PHP_VERSION=8.3 -t php-guard .

# 提取编译产物
docker run --rm -v $(pwd)/dist:/dist php-guard cp /build/target/release/libphp_guard.so /dist/
```

## 快速开始

### 使用 CLI 工具

```bash
# 初始化配置文件
php-guard init

# 生成随机密钥
php-guard generate-key

# 验证配置一致性
php-guard verify

# 加密文件
php-guard encrypt src/

# 检查加密状态
php-guard check src/

# 构建 PHP 扩展
php-guard build --release
```

### 使用 PHP 工具

```bash
# 加密文件
php tools/php-guard.php encrypt src/

# 检查加密状态
php tools/php-guard.php check src/

# 验证配置
php tools/verify-config.php

# 生成密钥
php tools/generate-key.php
```

## 文档

- [使用指南](documents/USAGE.md) - 详细安装和使用说明
- [架构设计](documents/ARCHITECTURE.md) - 技术架构和原理
- [Windows 支持](documents/WINDOWS_SUPPORT.md) - Windows 平台支持方案

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

## 项目结构

```
php-guard/
├── crates/
│   └── php-guard-cli/     # Rust CLI 工具
├── src/
│   ├── lib.rs             # 库入口
│   ├── config.rs          # 密钥配置
│   ├── crypto.rs          # 加密算法
│   ├── file_handler.rs    # 文件处理
│   ├── hooks.rs           # PHP hook
│   └── php_extension.rs   # PHP 扩展
├── tools/
│   ├── php-guard.php      # PHP 加密工具
│   ├── verify-config.php  # 配置验证
│   └── generate-key.php   # 密钥生成
├── .github/workflows/
│   ├── ci.yml             # 持续集成
│   └── release.yml        # 自动发布
├── Dockerfile             # Docker 构建
├── docker-compose.yml     # 多版本构建
└── Makefile               # 便捷命令
```

## Makefile 命令

```bash
make help              # 显示帮助
make build-cli         # 构建 CLI 工具
make build-release     # 构建扩展 (发布模式)
make install           # 安装扩展
make test              # 运行测试
make verify            # 验证配置
make generate-key      # 生成密钥
make docker-build      # Docker 构建
```

## 注意事项

1. **备份源码** - 加密前务必备份原始文件！
2. **密钥安全** - 生产环境务必使用自定义密钥
3. **选择性加密** - 建议只加密核心业务代码

## 开发

```bash
# 运行测试
cargo test

# 构建 CLI
cargo build -p php-guard-cli --release

# 构建扩展
cargo build --features php-extension --release

# 代码检查
cargo clippy
cargo fmt --check
```

## 致谢

- [tonyenc](https://github.com/lihancong/tonyenc) - 原始 C 实现
- [phper](https://github.com/phper-framework/phper) - Rust PHP 扩展框架

## 许可证

MIT License
