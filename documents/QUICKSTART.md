# PHP-Guard 快速入门指南

## 🚀 5 分钟快速开始

### 1. 生成配置（首次使用）

**Linux/macOS:**
```bash
./scripts/generate-key.sh
```

**Windows:**
```cmd
.\scripts\generate-key.bat
```

### 2. 构建 PHP 扩展

```bash
make build-release
```

### 3. 安装扩展

```bash
make install
```

### 4. 加密 PHP 文件

```bash
# 构建 CLI 工具
make build-cli

# 加密文件
make encrypt F=src/
```

### 5. 运行加密文件

```bash
php encrypted_file.php
```

---

## 📝 完整工作流程

### 开发环境设置

```bash
# 1. 克隆项目
git clone https://github.com/yourname/php-guard.git
cd php-guard

# 2. 生成配置
./scripts/generate-key.sh

# 3. 构建所有组件
make build-release  # 扩展
make build-cli      # CLI 工具

# 4. 安装扩展
make install

# 5. 验证安装
php -m | grep php_guard
```

### 日常使用

```bash
# 加密新文件
make encrypt F=path/to/file.php

# 检查加密状态
make check F=src/

# 运行加密文件
php encrypted_file.php
```

### 生产部署

```bash
# 1. 生成生产配置（独立环境）
./scripts/generate-key.sh

# 2. 备份配置文件
cp -r .php-guard/ ~/backup/php-guard-config/

# 3. 构建生产版本
make build-release

# 4. 加密所有文件
make encrypt F=src/

# 5. 部署
# - 复制加密文件到生产服务器
# - 复制扩展文件到生产服务器
# - 安装扩展
```

---

## ⚙️ 常用命令速查

| 操作 | 命令 |
|------|------|
| 生成配置 | `./scripts/generate-key.sh` |
| 构建扩展 | `make build-release` |
| 构建CLI | `make build-cli` |
| 安装扩展 | `make install` |
| 加密文件 | `make encrypt F=<path>` |
| 检查文件 | `make check F=<path>` |
| 运行测试 | `make test` |
| 代码检查 | `make lint` |
| 清理构建 | `make clean` |
| 查看帮助 | `make help` |

---

## 🔧 Makefile 命令详解

### 配置命令

```bash
# 生成加密密钥和配置
make generate-key
```

### 构建命令

```bash
# 开发模式构建扩展
make build

# 发布模式构建扩展（推荐）
make build-release

# 构建 CLI 工具
make build-cli
```

### CLI 工具命令

```bash
# 加密文件或目录
make encrypt F=example.php
make encrypt F=src/

# 检查加密状态
make check F=example.php
make check F=src/
```

### Docker 命令

```bash
# Docker 构建（默认 PHP 8.3）
make docker-build

# Docker 构建指定版本
make docker-build V=7.4

# Docker 测试
make docker-test V=8.3
```

### 测试命令

```bash
# 运行 Rust 测试
make test

# 代码检查
make lint
```

---

## 🔐 安全最佳实践

### 1. 配置文件管理

```bash
# 设置配置文件权限
chmod 600 .php-guard/config.env

# 备份配置文件
tar -czf php-guard-config-$(date +%Y%m%d).tar.gz .php-guard/

# 不要提交配置文件
# .gitignore 已包含 .php-guard/
```

### 2. 密钥轮换

```bash
# 1. 备份当前配置
cp -r .php-guard/ .php-guard.backup/

# 2. 生成新密钥
./scripts/generate-key.sh

# 3. 重新构建和安装
make clean build-release install

# 4. 重新加密所有文件
make encrypt F=src/

# 5. 删除备份（确认无误后）
rm -rf .php-guard.backup/
```

### 3. 生产环境检查清单

- [ ] 使用独立的生产密钥配置
- [ ] 配置文件权限设置为 600
- [ ] 备份配置文件到安全位置
- [ ] 只加密核心业务代码
- [ ] 测试加密文件在生产环境运行正常
- [ ] 文档记录密钥轮换流程

---

## 🐛 常见问题快速解决

### 扩展无法加载

```bash
# 检查扩展文件
ls -la $(php-config --extension-dir)/php_guard.so

# 检查错误信息
php -d extension=php_guard -v 2>&1

# 重新安装
make install
```

### 加密文件无法运行

```bash
# 1. 确认扩展已加载
php -m | grep php_guard

# 2. 确认使用相同密钥
cat .php-guard/config.env

# 3. 检查文件加密状态
make check F=file.php

# 4. 重新加密
make encrypt F=file.php
```

### 密钥丢失

⚠️ **密钥丢失后旧加密文件无法解密！**

恢复步骤：
1. 找到原始未加密的源文件
2. 重新生成配置：`./scripts/generate-key.sh`
3. 重新构建扩展：`make build-release install`
4. 重新加密文件：`make encrypt F=src/`

---

## 📚 相关文档

- [完整使用指南](USAGE.md) - 详细的使用说明
- [架构设计](ARCHITECTURE.md) - 技术架构说明
- [重构报告](REFACTORING_REPORT.md) - 最新变更说明
- [Windows 支持](WINDOWS_SUPPORT.md) - Windows 平台说明

---

## 💡 提示

1. **首次使用**: 必须先运行 `./scripts/generate-key.sh` 生成配置
2. **配置文件**: `.php-guard/config.env` 包含密钥，不要提交到版本控制
3. **备份**: 定期备份配置文件到安全位置
4. **生产环境**: 使用独立配置，不要使用开发环境密钥
5. **性能**: 加密文件配合 OPcache 使用性能更好

---

## 🆘 获取帮助

```bash
# 查看 Makefile 帮助
make help

# 查看 CLI 工具帮助
./target/release/php-guard-cli --help

# 查看扩展信息
php -d extension=php_guard -r "echo php_guard_version();"
```

**问题反馈:**
- GitHub Issues: https://github.com/yourname/php-guard/issues
- GitHub Discussions: https://github.com/yourname/php-guard/discussions
