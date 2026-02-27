# PHP-Guard Makefile

.PHONY: all build build-cli build-release install test clean help generate-key encrypt check cross-build

help:
	@echo "PHP-Guard 构建命令"
	@echo ""
	@echo "配置:"
	@echo "  make generate-key    - 生成加密密钥和配置"
	@echo ""
	@echo "构建:"
	@echo "  make build           - 开发模式构建扩展"
	@echo "  make build-release   - 发布模式构建扩展 (推荐)"
	@echo "  make build-cli       - 构建 CLI 工具"
	@echo "  make build-all       - 构建所有组件"
	@echo "  make install         - 安装扩展到 PHP"
	@echo ""
	@echo "交叉编译 CLI:"
	@echo "  make cross-build            - 交叉编译所有平台 CLI"
	@echo "  make cross-build-linux-x64  - Linux x64 CLI"
	@echo "  make cross-build-linux-arm64 - Linux ARM64 CLI (需要 zig)"
	@echo "  make cross-build-windows-x64 - Windows x64 CLI (需要 mingw-w64)"
	@echo ""
	@echo "CLI 工具 (需要先运行 make build-cli):"
	@echo "  make encrypt F=      - 加密文件 (F=路径)"
	@echo "  make check F=        - 检查加密状态 (F=路径)"
	@echo ""
	@echo "测试:"
	@echo "  make test            - 运行 Rust 测试"
	@echo "  make test-core       - 测试核心库"
	@echo "  make lint            - 代码检查"
	@echo ""
	@echo "清理:"
	@echo "  make clean           - 清理构建产物"

# ============================================
# 配置
# ============================================
generate-key:
	@echo "生成加密密钥和配置..."
	@if command -v bash >/dev/null 2>&1; then \
		bash scripts/generate-key.sh; \
	else \
		echo "错误: 需要 bash 环境"; \
		echo "Windows 用户请运行: scripts\\generate-key.bat"; \
		exit 1; \
	fi

# ============================================
# 构建
# ============================================
build:
	cargo build -p php-guard-ext

build-release:
	cargo build -p php-guard-ext --release

build-cli:
	cargo build -p php-guard-cli --release

build-core:
	cargo build -p php-guard-core --release

build-all: build-release build-cli
	@echo "所有组件构建完成"

# ============================================
# CLI 工具 (Rust)
# ============================================
CLI := target/release/php-guard-cli

encrypt: build-cli
ifndef F
	@echo "用法: make encrypt F=path/to/file.php"
	@echo "      make encrypt F=src/"
	@exit 1
endif
	@$(CLI) encrypt $(F)

check: build-cli
ifndef F
	@echo "用法: make check F=path/to/file.php"
	@echo "      make check F=src/"
	@exit 1
endif
	@$(CLI) check $(F)

# ============================================
# 安装
# ============================================
install: build-release
	@echo "安装扩展..."
	sudo cp target/release/libphp_guard_ext.so $$(php-config --extension-dir)/php_guard.so
	@echo "扩展已安装到: $$(php-config --extension-dir)/php_guard.so"
	@echo ""
	@echo "启用扩展 (选择一种方式):"
	@echo "  临时: php -d extension=php_guard script.php"
	@echo "  永久: echo 'extension=php_guard.so' | sudo tee /etc/php/conf.d/php_guard.ini"

# ============================================
# 测试
# ============================================
test:
	cargo test

test-core:
	cargo test -p php-guard-core

lint:
	cargo clippy -- -D warnings
	cargo fmt -- --check

# ============================================
# 清理
# ============================================
clean:
	cargo clean
	rm -rf dist/
	rm -f rustc-ice-*.txt
