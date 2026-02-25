# PHP-Guard 使用指南

## 目录

1. [快速开始](#快速开始)
2. [安装方式](#安装方式)
3. [配置管理](#配置管理)
4. [CLI 工具](#cli-工具)
5. [加密文件](#加密文件)
6. [运行加密文件](#运行加密文件)
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

# 1. 生成密钥配置
./scripts/generate-key.sh  # Linux/macOS
# 或
.\scripts\generate-key.bat  # Windows

# 2. 构建扩展
make build-release

# 3. 安装扩展
make install

# 4. 构建 CLI 工具
make build-cli

# 5. 加密文件
make encrypt F=src/
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

# 生成配置
./scripts/generate-key.sh

# 编译
make build-release
```

#### macOS

```bash
# 安装依赖
brew install php llvm

# 生成配置
./scripts/generate-key.sh

# 编译
export LIBCLANG_PATH=/usr/local/opt/llvm/lib
make build-release
```

#### Windows

```cmd
# 使用 WSL (推荐)
wsl bash scripts/generate-key.sh
wsl make build-release

# 或使用 Git Bash
bash scripts/generate-key.sh
cargo build --features php-extension --release
```

### 方式三：Docker 构建

```bash
# 构建指定 PHP 版本
make docker-build V=8.3

# 或手动运行
docker build --build-arg PHP_VERSION=8.3 -t php-guard .
```

---

## 配置管理

### 生成配置

首次使用需要生成加密密钥和头部标识：

**Linux/macOS:**
```bash
./scripts/generate-key.sh
```

**Windows:**
```cmd
.\scripts\generate-key.bat
```

生成的配置文件位于 `.php-guard/config.env`，包含：

```bash
# PHP-Guard 配置文件
export PHP_GUARD_KEY="a5c31b44b3ba19388fa082bbe1725c15ddd7319a3daca6c7451adec42cca6009"
export PHP_GUARD_HEADER="ca167d2f4529a4fa6168a939ad9d562e"
```

### 配置说明

- `PHP_GUARD_KEY`: 256位加密密钥（32字节，64个十六进制字符）
- `PHP_GUARD_HEADER`: 128位文件头部标识（16字节，32个十六进制字符）

### 自动配置

构建时 `build.rs` 会自动：

1. 检查 `.php-guard/config.env` 是否存在
2. 如果存在，读取配置
3. 如果不存在，生成随机配置并保存
4. 生成 Rust 代码到 `$OUT_DIR/config_generated.rs`

### 安全注意事项

⚠️ **重要提示:**

1. **不要提交配置文件**
   - `.php-guard/` 目录已添加到 `.gitignore`
   - 确保不会意外提交

2. **备份配置文件**
   - 在安全的地方备份 `.php-guard/config.env`
   - 丢失配置将无法解密已加密的文件

3. **生产环境配置**
   - 使用不同的密钥
   - 限制配置文件访问权限：`chmod 600 .php-guard/config.env`
   - 定期更换密钥

### 更换密钥

```bash
# 1. 生成新密钥
./scripts/generate-key.sh

# 2. 重新构建扩展
make build-release

# 3. 重新安装扩展
make install

# 4. 重新加密所有 PHP 文件
make encrypt F=src/
```

⚠️ **注意:** 更换密钥后，旧的加密文件将无法解密！

---

## CLI 工具

PHP-Guard 提供了简洁的 Rust CLI 工具。

### 构建 CLI

```bash
make build-cli
# 或
cargo build -p php-guard-cli --release
```

### 命令概览

```bash
./target/release/php-guard-cli --help
```

| 命令 | 说明 |
|------|------|
| `encrypt` | 加密文件或目录 |
| `check` | 检查文件加密状态 |

### 加密文件

```bash
# 加密单个文件
./target/release/php-guard-cli encrypt example.php

# 加密目录
./target/release/php-guard-cli encrypt src/

# 加密多个目标
./target/release/php-guard-cli encrypt app/ config/ routes/

# 指定输出目录
./target/release/php-guard-cli encrypt src/ --output dist/
```

### 检查加密状态

```bash
# 检查单个文件
./target/release/php-guard-cli check example.php

# 检查目录
./target/release/php-guard-cli check src/
```

### 使用 Makefile

```bash
# 加密文件
make encrypt F=example.php
make encrypt F=src/

# 检查文件
make check F=example.php
make check F=src/
```

---

## 加密文件

### 使用 CLI 工具（推荐）

```bash
# 加密单个文件
./target/release/php-guard-cli encrypt example.php

# 加密目录
./target/release/php-guard-cli encrypt src/

# 输出到指定目录
./target/release/php-guard-cli encrypt src/ --output dist/
```

### 使用 Makefile

```bash
# 加密文件或目录
make encrypt F=example.php
make encrypt F=src/

# 检查加密状态
make check F=src/
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

## PHP API

### php_guard_encode()

加密字符串内容。

```php
string php_guard_encode(string $content)
```

**示例:**
```php
<?php
$content = "<?php echo 'Hello, World!';";
$encrypted = php_guard_encode($content);
file_put_contents('encrypted.php', $encrypted);
```

### php_guard_is_encrypted()

检查内容是否已加密。

```php
bool php_guard_is_encrypted(string $content)
```

**示例:**
```php
<?php
$content = file_get_contents('some.php');
if (php_guard_is_encrypted($content)) {
    echo "文件已加密";
} else {
    echo "文件未加密";
}
```

### php_guard_version()

获取扩展版本号。

```php
string php_guard_version()
```

**示例:**
```php
<?php
echo php_guard_version(); // "0.1.0"
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
2. 确认使用了相同的密钥配置
3. 检查 PHP 版本兼容性
4. 确认文件确实被加密：`make check F=file.php`

### Q: 密钥丢失了怎么办？

如果密钥丢失：
- 旧加密文件无法解密
- 需要使用原始源文件重新加密
- 建议定期备份配置文件

### Q: 如何更换密钥？

```bash
# 1. 生成新密钥
./scripts/generate-key.sh

# 2. 重新构建扩展
make build-release
make install

# 3. 重新加密所有文件
make encrypt F=src/
```

⚠️ **注意:** 更换密钥后，旧加密文件将无法使用！

### Q: Windows 支持？

目前 phper 框架不支持 Windows 原生编译。推荐方案：

1. **WSL** - 在 WSL 中编译和使用（推荐）
2. **Docker** - 使用 Docker 构建 Linux 版本
3. **预编译版本** - 从 Releases 下载（如果提供）

详见 [Windows 支持方案](WINDOWS_SUPPORT.md)。

### Q: OPcache 兼容性？

完全兼容。加密文件解密后由 PHP 编译，OPcache 正常缓存。

### Q: 性能影响？

- 解密开销：O(n)
- 实测影响：< 1%
- 建议：配合 OPcache 使用

### Q: 如何确认配置正确？

```bash
# 1. 检查配置文件存在
ls -la .php-guard/config.env

# 2. 检查配置内容
cat .php-guard/config.env

# 3. 重新构建确保配置生效
make clean
make build-release
```

---

## 最佳实践

### 1. 备份源码

```bash
# 加密前备份
cp -r src/ src_backup/

# 或使用版本控制
git add . && git commit -m "Before encryption"
```

### 2. 备份配置文件

```bash
# 备份密钥配置
cp .php-guard/config.env .php-guard/config.env.backup

# 存储到安全位置
cp .php-guard/config.env ~/secure-backup/php-guard-config.env
```

### 3. 选择性加密

```bash
# 只加密核心业务代码
./target/release/php-guard-cli encrypt app/Services/ app/Models/

# 跳过框架和第三方库
```

### 4. 使用 Makefile

```makefile
# 项目 Makefile
.PHONY: encrypt deploy

encrypt:
	./target/release/php-guard-cli encrypt src/ --output dist/

deploy: encrypt
	rsync -avz dist/ user@server:/var/www/app/
```

### 5. CI/CD 集成

```yaml
# .github/workflows/deploy.yml
- name: Load configuration
  run: |
    # 从 secrets 加载配置
    mkdir -p .php-guard
    echo "$PHP_GUARD_CONFIG" > .php-guard/config.env
  env:
    PHP_GUARD_CONFIG: ${{ secrets.PHP_GUARD_CONFIG }}

- name: Build extension
  run: make build-release

- name: Encrypt PHP files
  run: make encrypt F=src/

- name: Deploy
  run: rsync -avz dist/ ${{ secrets.DEPLOY_TARGET }}
```

### 6. 安全管理

```bash
# 设置配置文件权限
chmod 600 .php-guard/config.env

# 定期更换密钥（建议每季度）
./scripts/generate-key.sh
make clean build-release install
make encrypt F=src/
```

### 7. 测试流程

```bash
# 1. 开发环境测试
make build-release
make install
php -d extension=php_guard test.php

# 2. 加密测试
make encrypt F=test.php
php -d extension=php_guard test.php

# 3. 生产环境部署
make encrypt F=src/
# 部署加密文件和扩展
```

---

## 技术支持

- 问题反馈：[GitHub Issues](https://github.com/yourname/php-guard/issues)
- 功能请求：[GitHub Discussions](https://github.com/yourname/php-guard/discussions)
- 文档：[项目文档](documents/)

---

## 相关文档

- [架构设计](ARCHITECTURE.md) - 技术架构和原理
- [项目重构报告](REFACTORING_REPORT.md) - 最新重构说明
- [Windows 支持](WINDOWS_SUPPORT.md) - Windows 平台支持方案
- [实施报告](IMPLEMENTATION_REPORT.md) - 详细实施文档
- [WSL 测试报告](WSL_TEST_REPORT.md) - 测试验证报告
