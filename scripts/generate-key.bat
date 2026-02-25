@echo off
REM 生成 PHP-Guard 加密密钥和头部标识

setlocal EnableDelayedExpansion

echo PHP-Guard 密钥生成工具
echo ========================

REM 配置文件路径
if "%PHP_GUARD_CONFIG_DIR%"=="" set PHP_GUARD_CONFIG_DIR=.php-guard
set CONFIG_DIR=%PHP_GUARD_CONFIG_DIR%
set CONFIG_FILE=%CONFIG_DIR%\config.env

REM 创建配置目录
if not exist "%CONFIG_DIR%" mkdir "%CONFIG_DIR%"

REM 检查是否已存在配置
if exist "%CONFIG_FILE%" (
    echo 警告: 配置文件已存在: %CONFIG_FILE%
    set /p OVERWRITE="是否覆盖现有配置? (y/N): "
    if /i not "!OVERWRITE!"=="y" (
        echo 操作已取消
        exit /b 0
    )
)

REM 生成随机密钥 (32字节 = 64个十六进制字符)
echo 生成加密密钥...
for /f "delims=" %%i in ('powershell -Command "[BitConverter]::ToString([System.Security.Cryptography.RandomNumberGenerator]::GetBytes(32)).Replace('-','').ToLower()"') do set KEY=%%i

REM 生成随机头部标识 (16字节 = 32个十六进制字符)
echo 生成头部标识...
for /f "delims=" %%i in ('powershell -Command "[BitConverter]::ToString([System.Security.Cryptography.RandomNumberGenerator]::GetBytes(16)).Replace('-','').ToLower()"') do set HEADER=%%i

REM 保存配置到文件
(
    echo # PHP-Guard 配置文件
    echo # 由 generate-key.bat 自动生成
    echo # 生成时间: %DATE% %TIME%
    echo.
    echo REM 加密密钥 (256位)
    echo set PHP_GUARD_KEY=%KEY%
    echo.
    echo REM 文件头部标识 (128位)
    echo set PHP_GUARD_HEADER=%HEADER%
) > "%CONFIG_FILE%"

echo.
echo 配置生成成功!
echo.
echo 配置文件位置: %CONFIG_FILE%
echo.
echo 密钥信息:
echo   KEY:    %KEY%
echo   HEADER: %HEADER%
echo.
echo 使用方法:
echo   1. 加载配置: call %CONFIG_FILE%
echo   2. 构建扩展: cargo build --features php-extension --release
echo   3. 加密文件: .\target\release\php-guard.exe encrypt ^<file^>
echo.
echo 注意: 请妥善保管配置文件，不要提交到版本控制系统
echo.

endlocal
