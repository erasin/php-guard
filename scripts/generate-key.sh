#!/bin/bash
# 生成 PHP-Guard 加密密钥和头部标识

set -e

echo "PHP-Guard 密钥生成工具"
echo "========================"

# 配置文件路径
CONFIG_DIR="${PHP_GUARD_CONFIG_DIR:-./.php-guard}"
CONFIG_FILE="$CONFIG_DIR/config.env"

# 创建配置目录
mkdir -p "$CONFIG_DIR"

# 检查是否已存在配置
if [ -f "$CONFIG_FILE" ]; then
    echo "⚠️  配置文件已存在: $CONFIG_FILE"
    read -p "是否覆盖现有配置? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "操作已取消"
        exit 0
    fi
fi

# 生成随机密钥 (32字节)
echo "生成加密密钥..."
KEY=$(openssl rand -hex 32)

# 生成随机头部标识 (16字节)
echo "生成头部标识..."
HEADER=$(openssl rand -hex 16)

# 保存配置到文件
cat > "$CONFIG_FILE" << EOF
# PHP-Guard 配置文件
# 由 generate-key.sh 自动生成
# 生成时间: $(date -Iseconds)

# 加密密钥 (256位)
export PHP_GUARD_KEY="$KEY"

# 文件头部标识 (128位)
export PHP_GUARD_HEADER="$HEADER"
EOF

echo ""
echo "✅ 配置生成成功!"
echo ""
echo "配置文件位置: $CONFIG_FILE"
echo ""
echo "密钥信息:"
echo "  KEY:    $KEY"
echo "  HEADER: $HEADER"
echo ""
echo "使用方法:"
echo "  1. 加载配置: source $CONFIG_FILE"
echo "  2. 构建扩展: cargo build --features php-extension --release"
echo "  3. 加密文件: ./target/release/php-guard encrypt <file>"
echo ""
echo "⚠️  注意: 请妥善保管配置文件，不要提交到版本控制系统"
echo ""
