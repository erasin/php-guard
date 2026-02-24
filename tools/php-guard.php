<?php

/**
 * PHP-Guard 加密工具
 * 
 * 使用方法:
 *   php php-guard.php encrypt <file_or_directory> [<file_or_directory2> ...]
 *   php php-guard.php check <file>
 * 
 * 示例:
 *   php php-guard.php encrypt example.php
 *   php php-guard.php encrypt src/
 *   php php-guard.php encrypt file1.php file2.php src/ lib/
 *   php php-guard.php check encrypted.php
 */

// 加密头 - 必须与 src/config.rs 中的 HEADER 一致
const HEADER = [
    0x66, 0x88, 0xff, 0x4f,
    0x68, 0x86, 0x00, 0x56,
    0x11, 0x61, 0x16, 0x18,
];

// 加密密钥 - 必须与 src/config.rs 中的 KEY 一致
const KEY = [
    0x9f, 0x49, 0x52, 0x00,
    0x58, 0x9f, 0xff, 0x23,
    0x8e, 0xfe, 0xea, 0xfa,
    0xa6, 0x33, 0xf3, 0xc6,
];

function encode(string &$data): void {
    $len = strlen($data);
    $keyLen = count(KEY);
    $p = 0;
    $key = KEY;
    
    for ($i = 0; $i < $len; ++$i) {
        if ($i & 1) {
            $p += $key[$p] + $i;
            $p %= $keyLen;
            $t = $key[$p];
            $data[$i] = chr(~(ord($data[$i]) ^ $t));
        }
    }
}

function isEncrypted(string $data): bool {
    $headerLen = count(HEADER);
    if (strlen($data) < $headerLen) {
        return false;
    }
    
    for ($i = 0; $i < $headerLen; $i++) {
        if (ord($data[$i]) !== HEADER[$i]) {
            return false;
        }
    }
    
    return true;
}

function encryptFile(string $filepath): bool {
    if (!file_exists($filepath)) {
        echo "文件不存在: $filepath\n";
        return false;
    }
    
    $content = file_get_contents($filepath);
    
    if ($content === false) {
        echo "无法读取文件: $filepath\n";
        return false;
    }
    
    if (isEncrypted($content)) {
        echo "已加密，跳过: $filepath\n";
        return true;
    }
    
    encode($content);
    
    $header = implode('', array_map('chr', HEADER));
    $encrypted = $header . $content;
    
    $result = file_put_contents($filepath, $encrypted);
    
    if ($result === false) {
        echo "写入失败: $filepath\n";
        return false;
    }
    
    echo "加密成功: $filepath\n";
    return true;
}

function checkFile(string $filepath): bool {
    if (!file_exists($filepath)) {
        echo "文件不存在: $filepath\n";
        return false;
    }
    
    $content = file_get_contents($filepath);
    
    if ($content === false) {
        echo "无法读取文件: $filepath\n";
        return false;
    }
    
    if (isEncrypted($content)) {
        echo "已加密: $filepath\n";
        return true;
    } else {
        echo "未加密: $filepath\n";
        return false;
    }
}

function processPath(string $path, callable $processor, string $extension = 'php'): int {
    $count = 0;
    
    if (is_file($path)) {
        if (pathinfo($path, PATHINFO_EXTENSION) === $extension) {
            if ($processor($path)) {
                $count++;
            }
        }
        return $count;
    }
    
    if (is_dir($path)) {
        $iterator = new RecursiveIteratorIterator(
            new RecursiveDirectoryIterator($path, RecursiveDirectoryIterator::SKIP_DOTS),
            RecursiveIteratorIterator::SELF_FIRST
        );
        
        foreach ($iterator as $file) {
            if ($file->isFile() && $file->getExtension() === $extension) {
                if ($processor($file->getPathname())) {
                    $count++;
                }
            }
        }
    }
    
    return $count;
}

function printUsage(): void {
    global $argv;
    echo "PHP-Guard 加密工具\n\n";
    echo "用法:\n";
    echo "  php {$argv[0]} encrypt <file_or_directory> [<file_or_directory2> ...]\n";
    echo "  php {$argv[0]} check <file>\n\n";
    echo "示例:\n";
    echo "  php {$argv[0]} encrypt example.php\n";
    echo "  php {$argv[0]} encrypt src/\n";
    echo "  php {$argv[0]} encrypt file1.php file2.php src/\n";
    echo "  php {$argv[0]} check encrypted.php\n";
}

function main(): int {
    global $argc, $argv;
    
    if ($argc < 3) {
        printUsage();
        return 1;
    }
    
    $command = $argv[1];
    $paths = array_slice($argv, 2);
    
    switch ($command) {
        case 'encrypt':
            $total = 0;
            foreach ($paths as $path) {
                $total += processPath($path, 'encryptFile');
            }
            echo "\n总计加密 $total 个文件\n";
            return 0;
            
        case 'check':
            $total = 0;
            foreach ($paths as $path) {
                processPath($path, function($file) use (&$total) {
                    if (checkFile($file)) {
                        $total++;
                    }
                });
            }
            echo "\n总计 $total 个文件已加密\n";
            return 0;
            
        default:
            echo "未知命令: $command\n\n";
            printUsage();
            return 1;
    }
}

exit(main());
