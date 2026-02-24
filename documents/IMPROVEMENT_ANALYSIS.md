# PHP-Guard 改进方案分析

## 当前状态

### 项目结构

```
php-guard/
├── src/
│   ├── lib.rs              # 库入口
│   ├── config.rs           # 密钥配置 (硬编码)
│   ├── crypto.rs           # 加密算法
│   ├── file_handler.rs     # 文件处理
│   ├── hooks.rs            # PHP hook
│   └── php_extension.rs    # PHP 扩展
├── tools/
│   └── php-guard.php       # 加密工具 (密钥硬编码)
├── examples/
│   └── encrypt_file.rs     # Rust 示例
└── Cargo.toml
```

### 当前工作流

```
1. 修改 src/config.rs 中的密钥
2. 修改 tools/php-guard.php 中对应的密钥
3. cargo build --features php-extension --release
4. 安装 .so 到 PHP 扩展目录
5. 使用 php tools/php-guard.php encrypt 加密文件
```

---

## 问题分析

### 1. 配置同步问题 (严重)

**问题描述:**
- 密钥在 `src/config.rs` 和 `tools/php-guard.php` 两处维护
- 容易忘记同步，导致加密文件无法解密
- 新用户容易出错

**当前代码:**

```rust
// src/config.rs
pub const HEADER: &[u8] = &[0x66, 0x88, 0xff, 0x4f, ...];
pub const KEY: &[u8] = &[0x9f, 0x49, 0x52, 0x00, ...];
```

```php
// tools/php-guard.php
const HEADER = [0x66, 0x88, 0xff, 0x4f, ...];
const KEY = [0x9f, 0x49, 0x52, 0x00, ...];
```

### 2. 构建复杂度高 (中等)

**问题描述:**
- 需要安装 PHP 开发环境、libclang
- 不同系统安装方式不同
- 新用户入门成本高

### 3. 加密工具分离 (轻微)

**问题描述:**
- 加密工具是 PHP 脚本
- 需要单独的 PHP 环境运行
- 与 Rust 扩展代码分离，逻辑重复

### 4. 版本管理 (轻微)

**问题描述:**
- 没有预编译版本
- 用户必须自行编译
- 多 PHP 版本需要多次编译

### 5. 密钥安全性 (轻微)

**问题描述:**
- 密钥编译进扩展二进制
- 可通过逆向工程提取

---

## 改进方案

### 方案 A: Rust CLI 工具 (推荐)

**核心思路:** 用 Rust 编写统一的命令行工具，复用加密库代码

```
php-guard/
├── src/
│   ├── lib.rs              # 核心库
│   ├── config.rs           # 配置 (从文件读取)
│   ├── crypto.rs           # 加密算法
│   └── ...
├── crates/
│   └── php-guard-cli/      # 新增: CLI 工具
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
└── config/
    └── php-guard.toml      # 配置文件
```

**配置文件示例 (php-guard.toml):**

```toml
[guard]
header = [0x66, 0x88, 0xff, 0x4f, 0x68, 0x86, 0x00, 0x56, 0x11, 0x61, 0x16, 0x18]
key = [0x9f, 0x49, 0x52, 0x00, 0x58, 0x9f, 0xff, 0x23, 0x8e, 0xfe, 0xea, 0xfa, 0xa6, 0x33, 0xf3, 0xc6]
```

**CLI 使用:**

```bash
# 生成默认配置
php-guard init

# 生成随机密钥
php-guard generate-key

# 加密文件
php-guard encrypt file.php
php-guard encrypt src/ --output dist/

# 检查加密状态
php-guard check file.php

# 编译扩展 (使用当前配置)
php-guard build --php-config $(which php-config)
```

**优点:**
- 单一配置源
- 无需 PHP 环境即可加密
- 代码复用，减少错误
- 统一的用户体验

**缺点:**
- 需要额外开发 CLI
- 增加项目复杂度

---

### 方案 B: 构建脚本自动化

**核心思路:** 使用构建脚本自动同步配置

```
php-guard/
├── src/
│   └── config.rs.in        # 配置模板
├── config/
│   └── guard.json          # 配置源
├── build.rs                # 构建脚本
└── tools/
    └── generate.php        # PHP 配置生成器
```

**配置文件 (config/guard.json):**

```json
{
  "header": [102, 136, 255, 79, 104, 134, 0, 86, 17, 97, 22, 24],
  "key": [159, 73, 82, 0, 88, 159, 255, 35, 142, 254, 234, 250, 166, 51, 243, 198]
}
```

**构建流程:**

```bash
# 1. 编辑配置
vim config/guard.json

# 2. 构建自动同步
cargo build --features php-extension
# build.rs 自动生成 src/config.rs 和 tools/php-guard.php
```

**优点:**
- 最小改动
- 保持现有结构

**缺点:**
- 仍需要 PHP 环境运行加密工具
- 构建脚本复杂

---

### 方案 C: Docker 构建环境

**核心思路:** 提供标准化构建环境

```dockerfile
# Dockerfile
FROM rust:1.85 AS builder
RUN apt-get update && apt-get install -y php-dev libclang-dev
COPY . /src
WORKDIR /src
RUN cargo build --features php-extension --release

FROM php:8.3-cli
COPY --from=builder /src/target/release/libphp_guard.so /usr/lib/php/modules/
```

**使用:**

```bash
# 构建扩展
docker build -t php-guard .

# 提取 .so 文件
docker run --rm -v $(pwd)/dist:/dist php-guard cp /usr/lib/php/modules/php_guard.so /dist/

# 使用加密工具
docker run --rm -v $(pwd)/src:/src php-guard php /src/tools/php-guard.php encrypt /src/
```

**优点:**
- 环境一致
- 无需本地安装依赖
- 可用于 CI/CD

**缺点:**
- Docker 知识要求
- 构建速度较慢

---

### 方案 D: GitHub Actions 自动发布

**核心思路:** 自动化构建和发布

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        php: ['8.0', '8.1', '8.2', '8.3', '8.4', '8.5']
        os: [ubuntu-22.04, ubuntu-20.04, macos-latest]
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v4
      - uses: shivammathur/setup-php@v2
        with:
          php-version: ${{ matrix.php }}
          extensions: none
      
      - name: Build
        run: cargo build --features php-extension --release
      
      - name: Upload
        uses: actions/upload-artifact@v4
        with:
          name: php_guard-php${{ matrix.php }}-${{ matrix.os }}
          path: target/release/libphp_guard.so
```

**优点:**
- 用户无需编译
- 支持多版本
- 自动化发布

**缺点:**
- 需要 GitHub 仓库
- 维护成本

---

## 方案对比

| 方案 | 开发难度 | 用户体验 | 维护成本 | 推荐度 |
|------|----------|----------|----------|--------|
| A. Rust CLI | 高 | ⭐⭐⭐⭐⭐ | 中 | ⭐⭐⭐⭐⭐ |
| B. 构建脚本 | 中 | ⭐⭐⭐ | 低 | ⭐⭐⭐ |
| C. Docker | 低 | ⭐⭐⭐ | 低 | ⭐⭐⭐⭐ |
| D. GitHub Actions | 中 | ⭐⭐⭐⭐⭐ | 中 | ⭐⭐⭐⭐ |
| A+D 组合 | 高 | ⭐⭐⭐⭐⭐ | 中 | ⭐⭐⭐⭐⭐ |

---

## 推荐实施路径

### 短期 (1-2 天)

1. **添加配置验证脚本**
   - 编写脚本检查 Rust 和 PHP 配置是否一致
   - 在构建前自动验证

2. **完善文档**
   - 添加详细的密钥配置说明
   - 添加常见错误排查指南

### 中期 (1 周)

1. **实现方案 C: Docker 构建环境**
   - 提供 Dockerfile
   - 提供一键构建脚本

2. **实现方案 D: GitHub Actions**
   - 自动构建多版本
   - 发布到 GitHub Releases

### 长期 (2-4 周)

1. **实现方案 A: Rust CLI 工具**
   - 独立的 `php-guard-cli` crate
   - 支持配置文件
   - 统一的命令行接口

2. **实现扩展编译功能**
   - CLI 内置编译扩展能力
   - 支持多个 PHP 版本

---

## 具体代码建议

### 1. 配置验证脚本

```bash
#!/bin/bash
# tools/verify-config.sh

set -e

echo "验证配置一致性..."

# 提取 Rust 配置
RUST_HEADER=$(grep -A1 'pub const HEADER' src/config.rs | tail -1 | sed 's/.*\[/[/' | sed 's/\].*/]/')
RUST_KEY=$(grep -A1 'pub const KEY' src/config.rs | tail -1 | sed 's/.*\[/[/' | sed 's/\].*/]/')

# 提取 PHP 配置
PHP_HEADER=$(grep -A5 'const HEADER' tools/php-guard.php | grep -E '0x[0-9a-f]{2}' | sed 's/.*\(0x[0-9a-f]*\).*/\1/' | tr '\n' ',' | sed 's/,$//' | sed 's/^/[/;s/$/]/')
PHP_KEY=$(grep -A20 'const KEY' tools/php-guard.php | grep -E '0x[0-9a-f]{2}' | sed 's/.*\(0x[0-9a-f]*\).*/\1/' | tr '\n' ',' | sed 's/,$//' | sed 's/^/[/;s/$/]/')

if [ "$RUST_HEADER" != "$PHP_HEADER" ]; then
    echo "错误: HEADER 配置不一致!"
    echo "Rust: $RUST_HEADER"
    echo "PHP:  $PHP_HEADER"
    exit 1
fi

if [ "$RUST_KEY" != "$PHP_KEY" ]; then
    echo "错误: KEY 配置不一致!"
    echo "Rust: $RUST_KEY"
    echo "PHP:  $PHP_KEY"
    exit 1
fi

echo "配置验证通过!"
```

### 2. 配置生成工具

```bash
#!/bin/bash
# tools/generate-config.sh

HEADER=$(openssl rand -hex 6 | fold -w2 | paste -sd',' - | sed 's/\(..\)/0x\1/g')
KEY=$(openssl rand -hex 8 | fold -w2 | paste -sd',' - | sed 's/\(..\)/0x\1/g')

cat > config/generated.toml << EOF
# 自动生成的配置 - $(date)
header = [$HEADER]
key = [$KEY]
EOF

echo "配置已生成到 config/generated.toml"
echo "请将其复制到 src/config.rs 和 tools/php-guard.php"
```

### 3. 一键构建脚本

```bash
#!/bin/bash
# scripts/build.sh

set -e

CONFIG_FILE="${1:-config/guard.json}"

if [ ! -f "$CONFIG_FILE" ]; then
    echo "配置文件不存在: $CONFIG_FILE"
    echo "请先运行 tools/generate-config.sh"
    exit 1
fi

echo "使用配置: $CONFIG_FILE"

# 生成 Rust 配置
# ... (解析 JSON 生成 config.rs)

# 生成 PHP 配置
# ... (解析 JSON 生成 php-guard.php)

# 编译
cargo build --features php-extension --release

# 输出
echo "构建完成: target/release/libphp_guard.so"
```

---

## 总结

### 最推荐的改进优先级

1. **[高优先级] 配置验证脚本** - 防止配置不一致
2. **[中优先级] Docker 构建环境** - 简化构建流程
3. **[中优先级] GitHub Actions** - 自动发布
4. **[低优先级] Rust CLI 工具** - 长期改进

### 当前可用但可改进的地方

1. **密钥配置** - 两处维护，需要手动同步
2. **构建环境** - 依赖复杂，可用 Docker 简化
3. **加密工具** - PHP 脚本，可替换为 Rust CLI
4. **发布方式** - 无预编译版本，可自动构建发布
