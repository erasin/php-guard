#!/usr/bin/env php
<?php

/**
 * PHP-Guard 密钥生成工具
 * 
 * 生成随机的加密头和密钥
 * 
 * 使用方法:
 *   php tools/generate-key.php [--header-length=12] [--key-length=16]
 */

function generateRandomBytes(int $length): array {
    $bytes = [];
    for ($i = 0; $i < $length; $i++) {
        $bytes[] = random_int(0, 255);
    }
    return $bytes;
}

function formatRustConst(array $bytes, string $name): string {
    $hexBytes = array_map(function($b) {
        return sprintf('0x%02x', $b);
    }, $bytes);
    
    $result = "pub const $name: &[u8] = &[\n";
    
    // 每行 4 个字节
    $chunks = array_chunk($hexBytes, 4);
    foreach ($chunks as $chunk) {
        $result .= "    " . implode(', ', $chunk) . ",\n";
    }
    $result .= "];\n";
    
    return $result;
}

function formatPhpConst(array $bytes, string $name): string {
    $hexBytes = array_map(function($b) {
        return sprintf('0x%02x', $b);
    }, $bytes);
    
    $result = "const $name = [\n";
    
    // 每行 4 个字节
    $chunks = array_chunk($hexBytes, 4);
    foreach ($chunks as $chunk) {
        $result .= "    " . implode(', ', $chunk) . ",\n";
    }
    $result .= "];\n";
    
    return $result;
}

function formatToml(array $header, array $key): string {
    $headerStr = '[' . implode(', ', array_map(function($b) {
        return sprintf('0x%02x', $b);
    }, $header)) . ']';
    
    $keyStr = '[' . implode(', ', array_map(function($b) {
        return sprintf('0x%02x', $b);
    }, $key)) . ']';
    
    return <<<TOML
# PHP-Guard 配置
# 生成时间: {{date}}

[guard]
header = $headerStr
key = $keyStr
TOML;
}

function main(): int {
    global $argc, $argv;
    
    // 解析参数
    $headerLength = 12;
    $keyLength = 16;
    
    for ($i = 1; $i < $argc; $i++) {
        if (preg_match('/^--header-length=(\d+)$/', $argv[$i], $matches)) {
            $headerLength = (int)$matches[1];
        } elseif (preg_match('/^--key-length=(\d+)$/', $argv[$i], $matches)) {
            $keyLength = (int)$matches[1];
        } elseif ($argv[$i] === '--help' || $argv[$i] === '-h') {
            echo "用法: php generate-key.php [选项]\n\n";
            echo "选项:\n";
            echo "  --header-length=N  加密头长度 (默认: 12)\n";
            echo "  --key-length=N     密钥长度 (默认: 16)\n";
            echo "  -h, --help         显示帮助\n";
            return 0;
        }
    }
    
    echo "PHP-Guard 密钥生成工具\n";
    echo "======================\n\n";
    
    // 生成随机字节
    $header = generateRandomBytes($headerLength);
    $key = generateRandomBytes($keyLength);
    
    echo "生成配置:\n";
    echo "- 加密头长度: $headerLength bytes\n";
    echo "- 密钥长度: $keyLength bytes\n\n";
    
    // Rust 格式
    echo "=== Rust (src/config.rs) ===\n";
    echo formatRustConst($header, 'HEADER');
    echo "\n";
    echo formatRustConst($key, 'KEY');
    echo "\n";
    
    // PHP 格式
    echo "=== PHP (tools/php-guard.php) ===\n";
    echo formatPhpConst($header, 'HEADER');
    echo "\n";
    echo formatPhpConst($key, 'KEY');
    echo "\n";
    
    // TOML 格式
    echo "=== TOML (config/php-guard.toml) ===\n";
    $toml = formatToml($header, $key);
    $toml = str_replace('{{date}}', date('Y-m-d H:i:s'), $toml);
    echo $toml . "\n";
    
    echo "\n请将上述配置复制到对应文件中。\n";
    echo "⚠️  重要: 确保两个文件使用相同的配置!\n";
    
    return 0;
}

exit(main());
