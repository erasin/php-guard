# PHP-Guard Docker 构建环境
# 支持多 PHP 版本构建和多架构

ARG PHP_VERSION=8.3
ARG RUST_VERSION=1.85
ARG TARGETPLATFORM
ARG BUILDPLATFORM

# ============================================
# Stage 1: 构建环境
# ============================================
FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-slim AS builder

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
        apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu; \
    fi \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# 复制项目文件
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY src ./src
COPY examples ./examples

# 构建参数：PHP 版本和配置路径
ARG PHP_VERSION=8.3
ARG PHP_CONFIG_PATH=/usr/bin/php-config

# 设置 PHP 配置路径
ENV PHP_CONFIG=${PHP_CONFIG_PATH}

# ============================================
# Stage 2: PHP 开发环境
# ============================================
FROM php:${PHP_VERSION}-cli AS php-builder

# 安装 PHP 开发工具
RUN apt-get update && apt-get install -y \
    $PHPIZE_DEPS \
    && rm -rf /var/lib/apt/lists/*

# ============================================
# Stage 3: 完整构建环境
# ============================================
FROM debian:bookworm-slim AS build-env

ARG PHP_VERSION=8.3

# 安装所有依赖
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libclang-dev \
    clang \
    wget \
    ca-certificates \
    php${PHP_VERSION}-dev \
    php${PHP_VERSION}-cli \
    && rm -rf /var/lib/apt/lists/*

# 安装 Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# 安装 aarch64 目标
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
        rustup target add aarch64-unknown-linux-gnu; \
    fi

WORKDIR /build

# 复制项目文件
COPY . .

# 构建
ARG TARGETPLATFORM
ENV CC=${TARGETPLATFORM == "linux/arm64" && "aarch64-linux-gnu-gcc" || "gcc"}
ENV CARGO_TARGET_${TARGETPLATFORM == "linux/arm64" && "AARCH64_UNKNOWN_LINUX_GNU" || ""}_LINKER=${TARGETPLATFORM == "linux/arm64" && "aarch64-linux-gnu-gcc" || "gcc"}
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
        cargo build --features php-extension --release --target aarch64-unknown-linux-gnu; \
    else \
        cargo build --features php-extension --release; \
    fi

# ============================================
# Stage 4: 运行时环境 (可选)
# ============================================
FROM php:${PHP_VERSION}-cli AS runtime

# 复制构建产物
ARG TARGETPLATFORM
COPY --from=build-env /build/target${TARGETPLATFORM == "linux/arm64" && "/aarch64-unknown-linux-gnu" || ""}/release/libphp_guard.so /usr/lib/php/modules/

# 启用扩展
RUN echo "extension=php_guard.so" > /usr/local/etc/php/conf.d/php_guard.ini

# 复制加密工具
COPY tools/php-guard.php /usr/local/bin/php-guard-tool
RUN chmod +x /usr/local/bin/php-guard-tool

WORKDIR /app

CMD ["php", "-a"]
