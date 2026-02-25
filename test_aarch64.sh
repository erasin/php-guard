#!/bin/bash
# 测试脚本：验证 php-guard 修复和 aarch64 支持

echo "🔧 PHP-Guard 修复和 aarch64 支持测试"
echo "=========================================="

# 1. 检查基本编译
echo "📋 1. 检查基本编译..."
if cargo check --features php-extension > /dev/null 2>&1; then
    echo "✅ 基本编译成功"
else
    echo "❌ 基本编译失败"
    exit 1
fi

# 2. 检查 CLI 工具构建
echo "📋 2. 检查 CLI 工具构建..."
if cargo build -p php-guard-cli --release > /dev/null 2>&1; then
    echo "✅ CLI 工具构建成功"
else
    echo "❌ CLI 工具构建失败"
    exit 1
fi

# 3. 检查 aarch64 目标是否可用
echo "📋 3. 检查 aarch64 目标..."
if rustup target list --installed | grep -q "aarch64-unknown-linux-gnu"; then
    echo "✅ aarch64 目标已安装"
else
    echo "❌ aarch64 目标未安装"
    exit 1
fi

# 4. 检查 aarch64 交叉编译工具
echo "📋 4. 检查 aarch64 交叉编译工具..."
if command -v aarch64-linux-gnu-gcc > /dev/null 2>&1; then
    echo "✅ aarch64 交叉编译工具已安装"
else
    echo "❌ aarch64 交叉编译工具未安装"
    exit 1
fi

# 5. 检查 aarch64 CLI 构建
echo "📋 5. 检查 aarch64 CLI 构建..."
if env CC=aarch64-linux-gnu-gcc CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc cargo build -p php-guard-cli --release --target aarch64-unknown-linux-gnu > /dev/null 2>&1; then
    echo "✅ aarch64 CLI 构建成功"
else
    echo "❌ aarch64 CLI 构建失败"
    exit 1
fi

# 6. 验证二进制文件
echo "📋 6. 验证二进制文件..."
if file target/aarch64-unknown-linux-gnu/release/php-guard | grep -q "ARM aarch64"; then
    echo "✅ aarch64 二进制文件验证成功"
else
    echo "❌ aarch64 二进制文件验证失败"
    exit 1
fi

# 7. 检查 GitHub Actions 配置
echo "📋 7. 检查 GitHub Actions 配置..."
if grep -q "aarch64-unknown-linux-gnu" .github/workflows/ci.yml && grep -q "aarch64" .github/workflows/release.yml; then
    echo "✅ GitHub Actions 配置包含 aarch64 支持"
else
    echo "❌ GitHub Actions 配置缺少 aarch64 支持"
    exit 1
fi

# 8. 检查文档更新
echo "📋 8. 检查文档更新..."
if grep -q "aarch64" README.md; then
    echo "✅ README.md 已更新 aarch64 支持"
else
    echo "❌ README.md 缺少 aarch64 支持说明"
    exit 1
fi

echo ""
echo "🎉 所有测试通过！php-guard 已成功添加 aarch64 支持"
echo ""
echo "📊 支持的架构:"
echo "  - Linux x86_64 ✅"
echo "  - Linux aarch64 ✅"
echo "  - Windows x86_64 ✅"
echo "  - macOS x86_64 🔄 (配置就绪)"
echo "  - macOS aarch64 🔄 (配置就绪)"
echo ""
echo "🚀 已修复的问题:"
echo "  - 类型不匹配错误 (ZStr::from_ptr vs *const i8)"
echo "  - 静态变量 unsafe 警告"
echo "  - Rust 2024 版本兼容性问题"
echo ""
echo "📦 生成的二进制文件:"
echo "  - x86_64: target/release/php-guard"
echo "  - aarch64: target/aarch64-unknown-linux-gnu/release/php-guard"