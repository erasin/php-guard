#!/usr/bin/env php
<?php

/**
 * PHP-Guard 配置验证工具
 * 
 * 验证 Rust 扩展和 PHP 加密工具的配置是否一致
 * 
 * 使用方法:
 *   php tools/verify-config.php
 */

const RUST_CONFIG_FILE = __DIR__ . '/../src/config.rs';
const PHP_CONFIG_FILE = __DIR__ . '/php-guard.php';

function parseRustConfig(string $file): array {
    $content = file_get_contents($file);
    
    // 解析 HEADER
    if (!preg_match('/pub const HEADER:\s*&\[u8\]\s*=\s*&\[([^\]]+)\]/s', $content, $matches)) {
        throw new Exception("无法解析 HEADER 配置");
    }
    $headerStr = $matches[1];
    preg_match_all('/0x([0-9a-fA-F]{2})/', $headerStr, $headerMatches);
    $header = array_map('hexdec', $headerMatches[1]);
    
    // 解析 KEY
    if (!preg_match('/pub const KEY:\s*&\[u8\]\s*=\s*&\[([^\]]+)\]/s', $content, $matches)) {
        throw new Exception("无法解析 KEY 配置");
    }
    $keyStr = $matches[1];
    preg_match_all('/0x([0-9a-fA-F]{2})/', $keyStr, $keyMatches);
    $key = array_map('hexdec', $keyMatches[1]);
    
    return ['header' => $header, 'key' => $key];
}

function parsePhpConfig(string $file): array {
    $content = file_get_contents($file);
    
    // 解析 HEADER
    if (!preg_match('/const HEADER\s*=\s*\[([^\]]+)\]/s', $content, $matches)) {
        throw new Exception("无法解析 PHP HEADER 配置");
    }
    $headerStr = $matches[1];
    preg_match_all('/0x([0-9a-fA-F]{2})/', $headerStr, $headerMatches);
    $header = array_map('hexdec', $headerMatches[1]);
    
    // 解析 KEY
    if (!preg_match('/const KEY\s*=\s*\[([^\]]+)\]/s', $content, $matches)) {
        throw new Exception("无法解析 PHP KEY 配置");
    }
    $keyStr = $matches[1];
    preg_match_all('/0x([0-9a-fA-F]{2})/', $keyStr, $keyMatches);
    $key = array_map('hexdec', $keyMatches[1]);
    
    return ['header' => $header, 'key' => $key];
}

function formatBytes(array $bytes): string {
    return implode(', ', array_map(function($b) {
        return sprintf('0x%02X', $b);
    }, $bytes));
}

function main(): int {
    echo "PHP-Guard 配置验证工具\n";
    echo "========================\n\n";
    
    // 检查文件存在
    if (!file_exists(RUST_CONFIG_FILE)) {
        echo "❌ 错误: Rust 配置文件不存在: " . RUST_CONFIG_FILE . "\n";
        return 1;
    }
    
    if (!file_exists(PHP_CONFIG_FILE)) {
        echo "❌ 错误: PHP 配置文件不存在: " . PHP_CONFIG_FILE . "\n";
        return 1;
    }
    
    echo "Rust 配置: " . RUST_CONFIG_FILE . "\n";
    echo "PHP 配置:  " . PHP_CONFIG_FILE . "\n\n";
    
    try {
        $rustConfig = parseRustConfig(RUST_CONFIG_FILE);
        $phpConfig = parsePhpConfig(PHP_CONFIG_FILE);
    } catch (Exception $e) {
        echo "❌ 解析错误: " . $e->getMessage() . "\n";
        return 1;
    }
    
    $hasError = false;
    
    // 比较 HEADER
    echo "--- HEADER ---\n";
    echo "Rust (" . count($rustConfig['header']) . " bytes): " . formatBytes($rustConfig['header']) . "\n";
    echo "PHP  (" . count($phpConfig['header']) . " bytes): " . formatBytes($phpConfig['header']) . "\n";
    
    if ($rustConfig['header'] === $phpConfig['header']) {
        echo "✅ HEADER 配置一致\n\n";
    } else {
        echo "❌ HEADER 配置不一致!\n\n";
        $hasError = true;
    }
    
    // 比较 KEY
    echo "--- KEY ---\n";
    echo "Rust (" . count($rustConfig['key']) . " bytes): " . formatBytes(array_slice($rustConfig['key'], 0, 8)) . " ...\n";
    echo "PHP  (" . count($phpConfig['key']) . " bytes): " . formatBytes(array_slice($phpConfig['key'], 0, 8)) . " ...\n";
    
    if ($rustConfig['key'] === $phpConfig['key']) {
        echo "✅ KEY 配置一致\n\n";
    } else {
        echo "❌ KEY 配置不一致!\n\n";
        $hasError = true;
    }
    
    // 总结
    echo "========================\n";
    if ($hasError) {
        echo "❌ 配置验证失败!\n";
        echo "\n请确保 src/config.rs 和 tools/php-guard.php 中的配置一致。\n";
        return 1;
    } else {
        echo "✅ 配置验证通过!\n";
        return 0;
    }
}

exit(main());
