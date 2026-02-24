# PHP-Guard 使用指南

## 目录

1. [快速开始](#快速开始)
2. [安装方式](#安装方式)
3. [CLI 工具](#cli-工具)
4. [加密文件](#加密文件)
5. [运行加密文件](#运行加密文件)
6. [配置说明](#配置说明)
7. [PHP API](#php-api)
8. [Docker 构建](#docker-构建)
9. [常见问题](#常见问题)
10. [最佳实践](#最佳实践)

---

## 快速开始

### 下载预编译版本

从 [Releases](https://github.com/yourname/php-guard/releases) 下载：

```bash
# 下载 CLI 工具
wget https://github.com/yourname/php-guard/releases/download/v0.1.0/php-guard-linux-x64.tar.gz
tar -xzf php-guard-linux-x64.tar.gz

# 下载 PHP 扩展
wget https://github.com/yourname/php-guard/releases/download/v0.1.0/php_guard-php8.3-linux-x64.tar.gz
tar -xzf php_guard-php8.3-linux-x64.tar.gz

# 安装扩展
sudo cp php_guard_php8.3_linux_x64.so $(php-config --extension-dir)/php_guard.so
```

### 从源码编译

```bash
# 克隆项目
git clone https://github.com/yourname/php-guard.git
cd php-guard

# 使用 Makefile 快速构建
make build-cli        # 构建 CLI 工具
make build-release    # 构建 PHP 扩展
make install          # 安装扩展
```

---

## 安装方式

### 方式一：下载预编译版本（推荐）

直接从 GitHub Releases 下载对应平台的预编译版本。

### 方式二：从源码编译

#### 环境要求

| 依赖 | 版本要求 |
|------|----------|
| Rust | 1.85+ |
| libclang | 9.0+ |
| PHP | 7.0 - 8.5 (NTS) |

#### Linux (Ubuntu/Debian)

```bash
# 安装依赖
sudo apt-get update
sudo apt-get install -y php-dev php-cli libclang-dev clang

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# 编译
make build-release
```

#### macOS

```bash
# 安装依赖
brew install php llvm

# 编译
export LIBCLANG_PATH=/usr/local/opt/llvm/lib
make build-release
```

### 方式三：Docker 构建

```bash
# 构建指定 PHP 版本
make docker-build V=8.3

# 或手动运行
docker build --build-arg PHP_VERSION=8.3 -t php-guard .
```

---

## CLI 工具

PHP-Guard 提供了统一的 Rust CLI 工具 `php-guard`。

### 构建 CLI

```bash
make build-cli
# 或
cargo build -p php-guard-cli --release
```

### 命令概览

```bash
php-guard --help
```

| 命令 | 说明 |
|------|------|
| `init` | 初始化配置文件 |
| `generate-key` | 生成随机密钥 |
| `verify` | 验证配置一致性 |
| `encrypt` | 加密文件 |
| `check` | 检查加密状态 |
| `build` | 构建 PHP 扩展 |

### 初始化配置

```bash
# 创建默认配置文件
php-guard init

# 生成的配置文件: php-guard.toml
```

### 生成密钥

```bash
# 生成默认长度密钥 (header: 12 bytes, key: 16 bytes)
php-guard generate-key

# 指定长度
php-guard generate-key --header-length 16 --key-length 32

# 保存到配置文件
php-guard generate-key --output php-guard.toml
```

### 验证配置

```bash
# 验证 src/config.rs 和 tools/php-guard.php 配置一致性
php-guard verify

# 指定文件路径
php-guard verify --rust-config src/config.rs --php-config tools/php-guard.php
```

### 加密文件

```bash
# 加密单个文件
php-guard encrypt example.php

# 加密目录
php-guard encrypt src/

# 加密多个目标
php-guard encrypt app/ config/ routes/

# 指定输出目录
php-guard encrypt src/ --output dist/

# 指定配置文件
php-guard encrypt src/ --config php-guard.toml
```

### 检查加密状态

```bash
# 检查单个文件
php-guard check example.php

# 检查目录
php-guard check src/
```

### 构建扩展

```bash
# 开发模式构建
php-guard build

# 发布模式构建
php-guard build --release

# 指定 php-config 路径
php-guard build --php-config /usr/bin/php-config8.3
```

---

## 加密文件

### 使用 CLI 工具（推荐）

```bash
# 加密单个文件
php-guard encrypt example.php

# 加密目录
php-guard encrypt src/

# 输出到指定目录
php-guard encrypt src/ --output dist/
```

### 使用 PHP 工具

```bash
# 加密单个文件
php tools/php-guard.php encrypt example.php

# 加密目录
php tools/php-guard.php encrypt src/

# 检查加密状态
php tools/php-guard.php check example.php
```

### 使用 PHP 函数

```php
<?php
$content = file_get_contents('source.php');
$encrypted = php_guard_encode($content);
file_put_contents('source.php', $encrypted);
```

---

## 运行加密文件

### CLI 模式

```bash
# 确保扩展已加载
php encrypted_script.php

# 临时加载扩展
php -d extension=php_guard encrypted_script.php
```

### FPM 模式

```bash
# 在 php.ini 中启用扩展
echo "extension=php_guard.so" | sudo tee /etc/php/conf.d/php_guard.ini

# 重启 PHP-FPM
sudo systemctl restart php-fpm
```

---

## 配置说明

### 使用 CLI 生成配置

```bash
# 生成密钥并输出 Rust 和 PHP 格式
php-guard generate-key
```

输出示例：

```
=== Rust (src/config.rs) ===
pub const HEADER: &[u8] = &[
    0x4e, 0xfc, 0x98, 0xbe,
    0x1e, 0x5c, 0x69, 0xaa,
    0x6c, 0x03, 0x62, 0x23,
];

pub const KEY: &[u8] = &[
    0xc8, 0x1d, 0xcf, 0xa7,
    0xed, 0xe3, 0xdc, 0xd9,
    0x8d, 0xcf, 0x2a, 0x04,
    0xda, 0xb4, 0x2e, 0x22,
];

=== PHP (tools/php-guard.php) ===
const HEADER = [
    0x4e, 0xfc, 0x98, 0xbe,
    0x1e, 0x5c, 0x69, 0xaa,
    0x6c, 0x03, 0x62, 0x23,
];

const KEY = [
    0xc8, 0x1d, 0xcf, 0xa7,
    0xed, 0xe3, 0xdc, 0xd9,
    0x8d, 0xcf, 0x2a, 0x04,
    0xda, 0xb4, 0x2e, 0x22,
];
```

### 手动配置

1. 编辑 `src/config.rs` 中的 `HEADER` 和 `KEY`
2. 同步修改 `tools/php-guard.php` 中对应的常量
3. 运行 `php-guard verify` 验证配置一致性
4. 重新编译：`make build-release`

### 使用 Makefile

```bash
# 生成密钥
make generate-key

# 验证配置
make verify

# 使用 CLI 工具
make cli-key        # 生成密钥
make cli-verify     # 验证配置
make cli-encrypt F=src/   # 加密
```

---

## PHP API

### php_guard_encode()

加密字符串内容。

```php
string php_guard_encode(string $content)
```

### php_guard_is_encrypted()

检查内容是否已加密。

```php
bool php_guard_is_encrypted(string $content)
```

### php_guard_version()

获取扩展版本号。

```php
string php_guard_version()
```

---

## Docker 构建

### 单版本构建

```bash
# 构建 PHP 8.3 版本
make docker-build V=8.3

# 或手动运行
docker build --build-arg PHP_VERSION=8.3 -t php-guard:php8.3 .
```

### 多版本构建

```bash
# 构建所有支持的版本
make docker-build-all

# 或使用 docker-compose
docker-compose build
```

### 提取编译产物

```bash
# 创建输出目录
mkdir -p dist

# 提取 .so 文件
docker run --rm -v $(pwd)/dist:/dist php-guard:php8.3 \
    cp /build/target/release/libphp_guard.so /dist/php_guard_php8.3.so
```

### 测试

```bash
# 测试扩展
make docker-test V=8.3
```

---

## 常见问题

### Q: 扩展加载失败？

```bash
# 检查扩展文件
ls -la $(php-config --extension-dir)/php_guard.so

# 查看详细错误
php -d extension=php_guard -v 2>&1
```

### Q: 加密文件无法运行？

1. 确认扩展已加载：`php -m | grep php_guard`
2. 验证配置一致性：`php-guard verify`
3. 检查 PHP 版本兼容性

### Q: Windows 支持？

目前 phper 框架不支持 Windows 原生编译。推荐方案：

1. **WSL** - 在 WSL 中编译和使用
2. **Docker** - 使用 Docker 构建 Linux 版本
3. **预编译版本** - 从 Releases 下载（如果提供）

详见 [Windows 支持方案](WINDOWS_SUPPORT.md)。

### Q: OPcache 兼容性？

完全兼容。加密文件解密后由 PHP 编译，OPcache 正常缓存。

### Q: 性能影响？

- 解密开销：O(n)
- 实测影响：< 1%
- 建议：配合 OPcache 使用

---

## 最佳实践

### 1. 备份源码

```bash
# 加密前备份
cp -r src/ src_backup/

# 或使用版本控制
git add . && git commit -m "Before encryption"
```

### 2. 选择性加密

```bash
# 只加密核心业务代码
php-guard encrypt app/Services/ app/Models/

# 跳过框架和第三方库
```

### 3. 使用 Makefile

```makefile
# 项目 Makefile
encrypt:
    php-guard encrypt src/ --output dist/

deploy: encrypt
    rsync -avz dist/ user@server:/var/www/app/

.PHONY: encrypt deploy
```

### 4. CI/CD 集成

```yaml
# .github/workflows/deploy.yml
- name: Encrypt PHP files
  run: |
    ./php-guard encrypt src/ --output dist/
    
- name: Deploy
  run: rsync -avz dist/ ${{ secrets.DEPLOY_TARGET }}
```

---

## 技术支持

- 问题反馈：[GitHub Issues](https://github.com/yourname/php-guard/issues)
- 功能请求：[GitHub Discussions](https://github.com/yourname/php-guard/discussions)
