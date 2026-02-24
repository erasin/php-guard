# PHP-Guard Makefile

.PHONY: all build build-cli build-release install test clean help verify generate-key encrypt check docker-build docker-test

# 默认目标
all: build-release

# 帮助信息
help:
	@echo "PHP-Guard 构建命令"
	@echo ""
	@echo "构建:"
	@echo "  make build           - 开发模式构建扩展"
	@echo "  make build-release   - 发布模式构建扩展 (推荐)"
	@echo "  make build-cli       - 构建 CLI 工具"
	@echo "  make install         - 安装扩展到 PHP"
	@echo ""
	@echo "CLI 工具 (需要先运行 make build-cli):"
	@echo "  make cli-init        - 初始化配置文件"
	@echo "  make cli-key         - 生成随机密钥"
	@echo "  make cli-verify      - 验证配置一致性"
	@echo "  make cli-encrypt F=  - 加密文件 (F=路径)"
	@echo "  make cli-check F=    - 检查加密状态 (F=路径)"
	@echo ""
	@echo "PHP 工具:"
	@echo "  make generate-key    - 生成新的密钥 (PHP)"
	@echo "  make verify          - 验证配置一致性 (PHP)"
	@echo "  make encrypt FILE=   - 加密文件 (PHP)"
	@echo "  make check FILE=     - 检查文件加密状态 (PHP)"
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
CLI := target/release/php-guard

cli-init: build-cli
	@$(CLI) init

cli-key: build-cli
	@$(CLI) generate-key

cli-verify: build-cli
	@$(CLI) verify

cli-encrypt: build-cli
ifndef F
	@echo "用法: make cli-encrypt F=path/to/file.php"
	@exit 1
endif
	@$(CLI) encrypt $(F)

cli-check: build-cli
ifndef F
	@echo "用法: make cli-check F=path/to/file.php"
	@exit 1
endif
	@$(CLI) check $(F)

# ============================================
# PHP 工具
# ============================================
verify:
	@php tools/verify-config.php

generate-key:
	@php tools/generate-key.php

encrypt:
ifndef FILE
	@echo "用法: make encrypt FILE=path/to/file.php"
	@exit 1
endif
	@php tools/php-guard.php encrypt $(FILE)

check:
ifndef FILE
	@echo "用法: make check FILE=path/to/file.php"
	@exit 1
endif
	@php tools/php-guard.php check $(FILE)

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
	rm -f php-guard.toml
