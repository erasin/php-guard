# php-config.ps1 - PowerShell version of php-config for Windows
# 
# Usage:
#   php-config.ps1 -Includes       Output PHP header include paths
#   php-config.ps1 -Version        Output PHP version
#   php-config.ps1 -ExtensionDir   Output extension directory
#   php-config.ps1 -PhpBinary      Output PHP binary path

param(
    [switch]$Includes,
    [switch]$Version,
    [switch]$ExtensionDir,
    [switch]$PhpBinary,
    [switch]$Prefix,
    [switch]$Ldflags,
    [switch]$Libs,
    [switch]$ConfigureOptions
)

# Set PHP development environment paths (can be overridden by environment variables)
$PhpDevPath = if ($env:PHP_DEV_PATH) { $env:PHP_DEV_PATH } else { "C:\tools\php\devel" }
$PhpIncludePath = if ($env:PHP_INCLUDE_PATH) { $env:PHP_INCLUDE_PATH } else { Join-Path $PhpDevPath "include" }

if ($Includes) {
    $includes = @(
        "-I$PhpIncludePath",
        "-I$PhpIncludePath\main",
        "-I$PhpIncludePath\Zend", 
        "-I$PhpIncludePath\TSRM",
        "-I$PhpIncludePath\ext",
        "-I$PhpIncludePath\ext\date\lib"
    )
    Write-Output ($includes -join " ")
}
elseif ($Version) {
    php -n -r "echo PHP_VERSION;"
}
elseif ($ExtensionDir) {
    php -n -r "echo ini_get('extension_dir');"
}
elseif ($PhpBinary) {
    (Get-Command php).Source
}
elseif ($Prefix) {
    Split-Path $PhpDevPath -Parent
}
elseif ($Ldflags -or $Libs -or $ConfigureOptions) {
    # Empty output for these options
    Write-Output ""
}
else {
    Write-Host "Usage: php-config.ps1 [OPTION]"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -Includes         Output include path flags"
    Write-Host "  -Version          Output PHP version"
    Write-Host "  -ExtensionDir     Output extension directory"
    Write-Host "  -PhpBinary        Output PHP binary path"
    Write-Host "  -Prefix           Output installation prefix"
    exit 1
}
