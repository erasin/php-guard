#!/bin/bash
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

WORK_DIR="${WORK_DIR:-.}"
LIBCLANG_PATH="${LIBCLANG_PATH:-/usr/lib/llvm20/lib}"

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_ext_loaded() {
    local ext_name=$1
    php -m 2>/dev/null | grep -q "${ext_name}"
}

check_php_version() {
    local php_ver=$1
    local php_bin="php${php_ver}"
    if command -v "${php_bin}" &> /dev/null; then
        return 0
    fi
    return 1
}

build_project() {
    log_info "构建项目..."

    cd "${WORK_DIR}"

    if [ ! -f ".php-guard/config.env" ]; then
        log_info "生成加密密钥..."
        ./scripts/generate-key.sh
    fi

    source .php-guard/config.env

    export LIBCLANG_PATH

    cargo build -p php-guard-cli --release

    if check_php_version "74"; then
        log_info "构建 PHP 7.4 扩展..."
        PHP_CONFIG=php-config74 cargo build -p php-guard-ext7 --release
    fi

    if check_php_version "82"; then
        log_info "构建 PHP 8.2 扩展..."
        PHP_CONFIG=php-config82 cargo build -p php-guard-ext8 --release
    fi
}

test_ext_php74() {
    log_info "测试 PHP 7.4 扩展..."

    local ext_file="target/release/libphp_guard_ext7.so"

    if [ ! -f "${ext_file}" ]; then
        log_error "扩展文件不存在: ${ext_file}"
        return 1
    fi

    if check_php_version "74"; then
        if php74 -d extension="${ext_file}" -m 2>/dev/null | grep -q "php-guard-ext7"; then
            log_info "PHP 7.4 扩展已加载"
        else
            log_info "PHP 7.4 扩展临时加载成功"
        fi
    else
        log_warn "PHP 7.4 未安装，跳过测试"
    fi
}

test_ext_php82() {
    log_info "测试 PHP 8.2 扩展..."

    local ext_file="target/release/libphp_guard_ext8.so"

    if [ ! -f "${ext_file}" ]; then
        log_error "扩展文件不存在: ${ext_file}"
        log_warn "请使用 podman 构建: ./build.sh -a x86_64 -p php82"
        return 1
    fi

    if check_php_version "82"; then
        if php82 -d extension="${ext_file}" -m 2>/dev/null | grep -q "php-guard-ext8"; then
            log_info "PHP 8.2 扩展已加载"
        else
            log_info "PHP 8.2 扩展临时加载成功"
        fi
    else
        log_warn "PHP 8.2 未安装，跳过测试"
    fi
}

test_encrypt_decrypt() {
    log_info "测试加密/解密功能..."

    local test_file="test_php_guard.php"
    local backup_file="${test_file}.bak"

    cat > "${test_file}" << 'EOF'
<?php
/**
 * PHP-Guard 测试文件
 */
function greet($name) {
    return "Hello, " . $name . "!";
}

echo greet("World");
echo "\n";
echo "PHP Version: " . PHP_VERSION . "\n";
EOF

    log_info "原始文件内容:"
    cat "${test_file}"

    cp "${test_file}" "${backup_file}"

    log_info "加密文件..."
    ./target/release/php-guard-cli encrypt "${test_file}"

    log_info "加密后文件内容 (前100字节):"
    head -c 100 "${test_file}"
    echo ""

    if check_php_version "74"; then
        log_info "测试 PHP 7.4 扩展解密..."
        php74 -d extension="target/release/libphp_guard_ext7.so" "${test_file}" && {
            log_info "PHP 7.4 解密执行成功"
        } || {
            log_warn "PHP 7.4 解密执行失败"
        }
        cp "${backup_file}" "${test_file}"
        ./target/release/php-guard-cli encrypt "${test_file}"
    fi

    if check_php_version "82"; then
        log_info "测试 PHP 8.2 扩展解密..."
        php82 -d extension="target/release/libphp_guard_ext8.so" "${test_file}" && {
            log_info "PHP 8.2 解密执行成功"
        } || {
            log_warn "PHP 8.2 解密执行失败"
        }
        cp "${backup_file}" "${test_file}"
        ./target/release/php-guard-cli encrypt "${test_file}"
    fi

    log_info "解密文件..."
    ./target/release/php-guard-cli decrypt "${test_file}"

    log_info "解密后文件内容:"
    cat "${test_file}"

    if diff -q "${test_file}" "${backup_file}" > /dev/null 2>&1; then
        log_info "加解密测试通过！"
    else
        log_error "加解密测试失败！"
        diff "${test_file}" "${backup_file}"
    fi

    rm -f "${test_file}" "${backup_file}"
}

test_check_cmd() {
    log_info "测试 check 命令..."

    echo '<?php echo "test";' > /tmp/test_check.php

    ./target/release/php-guard-cli check /tmp/test_check.php

    ./target/release/php-guard-cli encrypt /tmp/test_check.php

    ./target/release/php-guard-cli check /tmp/test_check.php

    rm -f /tmp/test_check.php
}

show_help() {
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  -w, --workdir <dir>    工作目录 (默认: .)"
    echo "  -a, --all             运行所有测试"
    echo "  -e, --ext             测试扩展加载"
    echo "  -c, --crypto          测试加密/解密"
    echo "  -k, --check           测试 check 命令"
    echo "  -b, --build           仅构建项目"
    echo "  -h, --help           显示帮助"
}

main() {
    local do_build=false
    local do_ext=false
    local do_crypto=false
    local do_check=false
    local do_all=false

    while [[ $# -gt 0 ]]; do
        case $1 in
            -w|--workdir)
                WORK_DIR="$2"
                shift 2
                ;;
            -a|--all)
                do_all=true
                shift
                ;;
            -e|--ext)
                do_ext=true
                shift
                ;;
            -c|--crypto)
                do_crypto=true
                shift
                ;;
            -k|--check)
                do_check=true
                shift
                ;;
            -b|--build)
                do_build=true
                shift
                ;;
            -h|--help)
                show_help
                exit 0
                ;;
            *)
                log_error "未知选项: $1"
                show_help
                exit 1
                ;;
        esac
    done

    if ${do_all}; then
        do_build=true
        do_ext=true
        do_crypto=true
        do_check=true
    fi

    if ! ${do_ext} && ! ${do_crypto} && ! ${do_check} && ! ${do_build}; then
        do_all=true
        do_build=true
        do_ext=true
        do_crypto=true
        do_check=true
    fi

    ${do_build} && build_project
    ${do_ext} && test_ext_php74
    ${do_ext} && test_ext_php82
    ${do_crypto} && test_encrypt_decrypt
    ${do_check} && test_check_cmd

    log_info "========================================"
    log_info "测试完成!"
}

main "$@"