# PHP-Guard Makefile

.PHONY: all build build-cli build-release install test clean help generate-key encrypt check docker-build docker-test

# Generate a list of available commands and their descriptions by parsing the Makefile.
help:
	@echo "命令:"
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%${width}s\033[0m: %s\n", $$1, $$2}'
	@echo ""

# 默认目标
all: help

# 帮助信息
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
	@echo "  make install         - 安装扩展到 PHP"
	@echo ""
	@echo "CLI 工具 (需要先运行 make build-cli):"
	@echo "  make encrypt F=      - 加密文件 (F=路径)"
	@echo "  make check F=        - 检查加密状态 (F=路径)"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-build    - Docker 构建 (PHP 8.3)"
	@echo "  make docker-build V=7.4  - 指定 PHP 版本"
	@echo "  make docker-test     - Docker 测试"
	@echo ""
	@echo "测试:"
	@echo "  make test            - 运行 Rust 测试"
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
	cargo build --features php-extension

build-release:
	cargo build --features php-extension --release

build-cli:
	cargo build -p php-guard-cli --release

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
	sudo cp target/release/libphp_guard.so $$(php-config --extension-dir)/php_guard.so
	@echo "扩展已安装到: $$(php-config --extension-dir)/php_guard.so"
	@echo ""
	@echo "启用扩展 (选择一种方式):"
	@echo "  临时: php -d extension=php_guard script.php"
	@echo "  永久: echo 'extension=php_guard.so' | sudo tee /etc/php/conf.d/php_guard.ini"

# ============================================
# Docker
# ============================================
PHP_VERSION ?= 8.3

docker-build:
	@echo "Docker 构建 PHP $(PHP_VERSION)..."
	docker build --build-arg PHP_VERSION=$(PHP_VERSION) -t php-guard:php$(PHP_VERSION) .
	@echo ""
	@echo "提取编译产物:"
	@echo "  mkdir -p dist"
	@echo "  docker run --rm -v $$(pwd)/dist:/dist php-guard:php$(PHP_VERSION) cp /build/target/release/libphp_guard.so /dist/php_guard_php$(PHP_VERSION).so"

docker-build-all:
	@echo "构建所有 PHP 版本..."
	@for v in 7.4 8.0 8.1 8.2 8.3 8.4; do \
		echo "=== PHP $$v ==="; \
		docker build --build-arg PHP_VERSION=$$v -t php-guard:php$$v . || true; \
	done

docker-test:
	docker run --rm php-guard:php$(PHP_VERSION) php -d extension=php_guard -r "echo 'Version: ' . php_guard_version() . PHP_EOL;"

# ============================================
# 测试
# ============================================
test:
	cargo test

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
