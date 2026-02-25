<?php
// 测试文件：验证 php-guard 扩展的各种功能
echo "PHP-Guard 扩展测试" . PHP_EOL;
echo "======================" . PHP_EOL;
// 测试版本函数
echo "版本: " . php_guard_version() . PHP_EOL;
// 测试加密检测函数
$content = "<?php echo 'Hello from encrypted file!';";
$encrypted = php_guard_encode($content);
echo "内容已加密" . PHP_EOL;
// 测试加密检查函数
if (php_guard_is_encrypted($encrypted)) {
    echo "加密检测成功" . PHP_EOL;
} else {
    echo "加密检测失败" . PHP_EOL;
}
// 测试解密
$decrypted = php_guard_decode($encrypted);
if ($decrypted === $content) {
    echo "解密成功: " . $decrypted . PHP_EOL;
} else {
    echo "解密失败" . PHP_EOL;
}
echo PHP_EOL . "测试完成！" . PHP_EOL;

