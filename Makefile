# PHP-Guard Makefile

.PHONY: all build install clean help

# ============================================
# 配置 (仅当配置不存在时生成)
# ============================================
config:
	@if [ ! -f .php-guard/config.env ]; then \
		echo "生成加密密钥和配置..."; \
		mkdir -p .php-guard; \
		KEY=$$(openssl rand -hex 32); \
		HEADER=$$(openssl rand -hex 16); \
		echo "export PHP_GUARD_KEY=\"$$KEY"" > .php-guard/config.env; \
		echo "export PHP_GUARD_HEADER="$$HEADER"" >> .php-guard/config.env; \
	fi

# ============================================
# 构建
# ============================================
export LIBCLANG_PATH = /usr/lib/llvm20/lib

build:
	cargo build --release

build-ext7:
	PHP_CONFIG=php-config74 cargo build -p php-guard-ext7 --release

build-ext8:
	PHP_CONFIG=php-config82 cargo build -p php-guard-ext8 --release

build-ext7-centos7:
	PHP_CONFIG=php-config74 cargo zigbuild -p php-guard-ext7 --release --target x86_64-unknown-linux-gnu.2.17

# ============================================
# 安装
# ============================================
install: build
	@echo "安装扩展..."
	sudo cp target/release/libphp_guard_ext7.so $$(php-config --extension-dir)/php_guard.so
	@echo "扩展已安装到: $$(php-config --extension-dir)/php_guard.so"
	cp target/release/php-guard-cli ~/.cargo/bin/

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
	rm -rf .php-guard
