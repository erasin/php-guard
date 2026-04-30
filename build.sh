#!/bin/bash
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

CRATE="php-guard-ext8"
OUTPUT_DIR="target/dist"

show_help() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -a, --arch <arch>      Target architecture: x86_64, aarch64, all (default: x86_64)"
    echo "  -p, --php <version>    PHP version: php74, php82, all (default: php82)"
    echo "  -o, --output <dir>     Output directory (default: dist)"
    echo "  -c, --crate <name>     Crate to build: php-guard-ext7, php-guard-ext8 (default: php-guard-ext8)"
    echo "  -h, --help             Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 -a x86_64 -p php82"
    echo "  $0 -a all -p all"
    echo "  $0 -a aarch64 -p php74 -o ./output"
}

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

ARCHS=("x86_64-unknown-linux-gnu")
PHP_VERSIONS=("8.2")
CRATE="php-guard-ext8"

while [[ $# -gt 0 ]]; do
    case $1 in
        -a|--arch)
            case $2 in
                x86_64) ARCHS=("x86_64-unknown-linux-gnu") ;;
                aarch64) ARCHS=("aarch64-unknown-linux-gnu") ;;
                all) ARCHS=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu") ;;
                *) log_error "Unknown architecture: $2"; exit 1 ;;
            esac
            shift 2
            ;;
        -p|--php)
            case $2 in
                php74) PHP_VERSIONS=("7.4") ;;
                php82) PHP_VERSIONS=("8.2") ;;
                all) PHP_VERSIONS=("7.4" "8.2") ;;
                *) log_error "Unknown PHP version: $2"; exit 1 ;;
            esac
            shift 2
            ;;
        -o|--output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        -c|--crate)
            CRATE="$2"
            shift 2
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

log_info "Building ${CRATE} for PHP ${PHP_VERSIONS[*]} on ${ARCHS[*]}"
log_info "Output directory: ${OUTPUT_DIR}"

mkdir -p "${OUTPUT_DIR}"

install_rust_target() {
    local target=$1
    if ! rustup target list --installed | grep -q "^${target}$"; then
        log_info "Installing Rust target: ${target}"
        rustup target add "${target}"
    fi
}

build_for_php() {
    local php_ver=$1
    local target=$2
    local php_image="php:${php_ver}-cli"

    log_info "----------------------------------------"
    log_info "Building for PHP ${php_ver} on ${target}"
    log_info "Image: ${php_image}"

    local ext_name="${CRATE}-${php_ver}-${target}.so"
    local container_workdir="/workspace"

    podman run --rm \
        -v "$(pwd):${container_workdir}" \
        -w "${container_workdir}" \
        --platform "linux/${target#*-linux-gnu}" \
        "${php_image}" \
        bash -c "
            set -e

            apt-get update && apt-get install -y \
                gcc \
                make \
                libclang-dev \
                clang \
                rustc \
                cargo \
                curl

            rustup target add ${target}

            cargo build -p ${CRATE} --target ${target} --release

            cp target/${target}/release/*.so ${OUTPUT_DIR}/${ext_name}
            echo 'Built: ${OUTPUT_DIR}/${ext_name}'
        "

    log_info "Built: ${ext_name}"
}

for PHP_VER in "${PHP_VERSIONS[@]}"; do
    for ARCH in "${ARCHS[@]}"; do
        build_for_php "${PHP_VER}" "${ARCH}"
    done
done

log_info "========================================"
log_info "Build complete!"
log_info "Output directory: ${OUTPUT_DIR}"
ls -lh "${OUTPUT_DIR}"/