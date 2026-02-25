# PHP-Guard CLI 工具更新报告

## 🎯 新增功能

### 1. 加密时自动创建备份文件

- 加密文件前自动创建 `.bak` 备份文件
- 如果备份文件已存在，会显示警告并跳过
- 用户可以安全地恢复原始文件
- 假设丢失备份文件时可以通过 `.bak` 文件恢复

### 2. 解密还原工具

- 新增 `decrypt` 命令，- 从 `.bak` 文件恢复原始内容
- 如果文件未加密，显示错误并跳过
- 支持自定义输出目录
- 智能处理文件扩展名（移除 `.bak`)

### 📝 使用说明

**加密:**
```bash
./target/release/php-guard-cli encrypt <file>
./target/release/php-guard-cli encrypt <directory>
./target/release/php-guard-cli encrypt <file> --output <output_dir>
```

**解密:**
```bash
./target/release/php-guard-cli decrypt <file>
./target/release/php-guard-cli decrypt <directory>
./target/release/php-guard-cli decrypt <file> --output <output_dir>
```

**检查:**
```bash
./target/release/php-guard-cli check <file>
```

## 🔧 技术实现

- **自动备份**: 加密时创建 `.bak` 文件
- **智能恢复**: 解密时从 `.bak` 恢复原始文件名
- **输出灵活**: 支持自定义输出目录
- **状态检查**: 检查加密状态
- **错误处理**: 官善的错误提示

- **输出路径** 支持 `-o` 或 `--output` 参数

## 📋 文代码变更

- `commands.rs` - 添加 `decrypt` 命令实现
- `main.rs` - 更新 CLI 娡型添加 `Decrypt` 变体
- `lib.rs` - 导出解密函数
- `USAGE.md` - 更新文档说明

- `GITHUB_ACTIONS_UPDATE.md` - 更新报告
- `QUICKSTART.md` - 添加快速入门指南

- `DOCUMENTATION_UPDATE.md` - 添加 CLI 工具更新报告

## ⚠️ 注意事项

- **首次加密**: 会自动创建 `.bak` 备份文件
- **备份文件**: `.bak` 文件会保留原始内容，- **解密还原**: 从 `.bak` 文件恢复原始文件
- **多次加密**: 如果文件已加密，会显示警告并跳过
- **文件管理**: 寏议妥善管理加密文件和避免意外覆盖

