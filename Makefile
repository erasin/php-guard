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
build:
	cargo build --release

# ============================================
# 安装
# ============================================
install: build
	@echo "安装扩展..."
	sudo cp target/release/libphp_guard_ext.so $$(php-config --extension-dir)/php_guard.so
	@echo "扩展已安装到: $$(php-config --extension-dir)/php_guard.so"

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
