@echo off
REM php-config.bat - 模拟 Linux php-config 命令 for Windows
REM 
REM 用法:
REM   php-config.bat --includes    输出 PHP 头文件包含路径
REM   php-config.bat --version     输出 PHP 版本
REM   php-config.bat --extension-dir  输出扩展目录
REM   php-config.bat --php-binary  输出 PHP 可执行文件路径

setlocal enabledelayedexpansion

REM 设置 PHP 开发环境路径 (可通过环境变量覆盖)
if not defined PHP_DEV_PATH (
    set "PHP_DEV_PATH=C:\tools\php\devel"
)

if not defined PHP_INCLUDE_PATH (
    set "PHP_INCLUDE_PATH=%PHP_DEV_PATH%\include"
)

REM 解析参数
set "arg=%1"

if "%arg%"=="--includes" (
    REM 输出格式: -I/path1 -I/path2 ...
    echo -I%PHP_INCLUDE_PATH% -I%PHP_INCLUDE_PATH%\main -I%PHP_INCLUDE_PATH%\Zend -I%PHP_INCLUDE_PATH%\TSRM -I%PHP_INCLUDE_PATH%\ext -I%PHP_INCLUDE_PATH%\ext\date\lib
) else if "%arg%"=="--version" (
    php -n -r "echo PHP_VERSION;"
) else if "%arg%"=="--extension-dir" (
    php -n -r "echo ini_get('extension_dir');"
) else if "%arg%"=="--php-binary" (
    where php
) else if "%arg%"=="--prefix" (
    echo %PHP_DEV_PATH%\..
) else if "%arg%"=="--configure-options" (
    echo.
) else if "%arg%"=="--ldflags" (
    echo.
) else if "%arg%"=="--libs" (
    echo.
) else (
    echo Usage: php-config [OPTION]
    echo.
    echo Options:
    echo   --includes         Output include path flags
    echo   --version          Output PHP version
    echo   --extension-dir    Output extension directory
    echo   --php-binary       Output PHP binary path
    echo   --prefix           Output installation prefix
    exit /b 1
)

endlocal
