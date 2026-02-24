# PHP-Guard Windows 支持方案分析

## 现状

### phper 框架限制

根据 [phper 官方文档](https://github.com/phper-framework/phper)，目前支持情况：

| 类别 | 项目 | 状态 |
|------|------|------|
| OS | Linux | ✅ |
| OS | macOS | ✅ |
| OS | Windows | ❌ |
| PHP | NTS | ✅ |
| PHP | ZTS | ❌ |

**原因分析：**
1. `phper-sys` 构建依赖 `php-config` 命令
2. Windows PHP 不提供 `php-config`
3. Windows PHP 头文件路径和配置方式不同

## Windows 支持方案

### 方案 1: WSL 构建 (推荐 - 当前可用)

使用 WSL 在 Windows 上构建 Linux 版本扩展。

**优点：**
- 无需额外开发
- 立即可用
- 与 Linux 完全兼容

**缺点：**
- 需要 WSL 环境
- 生成的是 Linux .so 文件
- 无法直接用于 Windows PHP

**使用方法：**
```bash
# 在 WSL 中
cargo build --features php-extension --release
# 生成 target/release/libphp_guard.so
```

### 方案 2: Docker 构建 (推荐 - CI/CD)

使用 Docker 容器构建 Linux 版本。

**优点：**
- 环境隔离
- 可用于 CI/CD
- 多版本构建

**缺点：**
- 需要 Docker
- 同样生成 Linux 版本

**使用方法：**
```bash
docker build -t php-guard --build-arg PHP_VERSION=8.3 .
docker run --rm -v $(pwd)/dist:/dist php-guard cp /build/target/release/libphp_guard.so /dist/
```

### 方案 3: MSYS2/MinGW 构建 (实验性)

在 Windows 上使用 MSYS2 环境构建。

**步骤：**
1. 安装 MSYS2
2. 安装 PHP 开发环境
3. 修改 phper-sys 构建脚本

**修改 phper-sys/build.rs:**
```rust
#[cfg(windows)]
fn get_php_includes() -> Vec<PathBuf> {
    let php_path = env::var("PHP_SRC_PATH")
        .expect("PHP_SRC_PATH not set");
    
    vec![
        PathBuf::from(&php_path),
        PathBuf::from(&php_path).join("main"),
        PathBuf::from(&php_path).join("Zend"),
        PathBuf::from(&php_path).join("TSRM"),
        PathBuf::from(&php_path).join("ext"),
    ]
}
```

**优点：**
- 原生 Windows 构建
- 生成 .dll 文件

**缺点：**
- 需要大量适配工作
- phper 框架可能不支持
- 维护成本高

### 方案 4: 纯 PHP 实现 (备选)

放弃 PHP 扩展，使用纯 PHP 解密方案。

**实现思路：**
```php
<?php
// php-guard-loader.php
stream_wrapper_register('phpguard', PHPGuardStream::class);

class PHPGuardStream {
    private $data;
    private $position = 0;
    
    public function stream_open($path, $mode, $options, &$opened_path) {
        // 读取加密文件
        $content = file_get_contents(substr($path, 10));
        
        // 解密
        $this->data = $this->decrypt($content);
        return true;
    }
    
    private function decrypt($data) {
        // 实现解密逻辑
        // ...
    }
}
```

**优点：**
- 跨平台
- 无需编译
- 易于部署

**缺点：**
- 性能较差
- 不够透明
- 需要修改代码加载方式

### 方案 5: 预编译 DLL 发布 (推荐 - 用户友好)

为常见 Windows PHP 版本预编译 DLL。

**实现步骤：**

1. 使用 GitHub Actions + Windows runner
2. 安装 PHP 开发环境
3. 手动配置 phper-sys
4. 构建并发布 DLL

**GitHub Actions 配置：**
```yaml
build-windows:
  runs-on: windows-latest
  strategy:
    matrix:
      php: ['7.4', '8.0', '8.1', '8.2', '8.3']
      arch: ['x64', 'x86']
      ts: ['ts', 'nts']
  
  steps:
    - uses: actions/checkout@v4
    
    - name: Setup PHP
      uses: szwacz/setup-php@v1
      with:
        php-version: ${{ matrix.php }}
        arch: ${{ matrix.arch }}
        ts: ${{ matrix.ts }}
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        target: ${{ matrix.arch }}-pc-windows-msvc
    
    - name: Configure PHP paths
      run: |
        echo "PHP_SRC_PATH=$env:PHP_SRC" >> $env:GITHUB_ENV
        echo "PHP_INCLUDES=$env:PHP_SRC\main;$env:PHP_SRC\Zend" >> $env:GITHUB_ENV
    
    - name: Build
      run: cargo build --features php-extension --release
```

**优点：**
- 用户无需编译
- 覆盖主流版本

**缺点：**
- 构建配置复杂
- 需要适配 phper

## 推荐方案组合

### 短期（立即可用）

1. **WSL + Linux 构建** - 开发者使用
2. **Docker 构建** - CI/CD 使用
3. **提供 Linux 预编译版本** - 用户下载

### 中期（优化体验）

1. **GitHub Actions 自动发布** - Linux/macOS 多版本
2. **纯 PHP 备选方案** - 作为降级选项

### 长期（完整支持）

1. **Windows DLL 预编译** - 需要适配 phper
2. **或 fork phper 添加 Windows 支持**

## 当前可用资源

### Windows 环境
```
E:/erasin/phpEnv/php/php7.4/
├── php.exe
├── php7ts.dll (线程安全)
├── ext/
│   └── *.dll
└── dev/
    ├── php.exe
    └── php7ts.lib
```

### 在 WSL 中访问 Windows PHP

```bash
# 访问 Windows PHP
/mnt/e/erasin/phpEnv/php/php7.4/php.exe -v

# 输出
PHP 7.4.x (cli) ...
```

### 使用 Windows PHP 测试加密文件

```bash
# 在 WSL 中加密
php tools/php-guard.php encrypt test.php

# 在 Windows 中运行（需要安装扩展）
/mnt/e/erasin/phpEnv/php/php7.4/php.exe -d extension=php_guard test.php
```

## 纯 PHP 实现方案

如果 Windows 支持困难，可以提供一个纯 PHP 的加载器：

```php
<?php
// php-guard-loader.php

class PHPGuardLoader {
    private const HEADER = [0x66, 0x88, 0xff, ...];
    private const KEY = [0x9f, 0x49, 0x52, ...];
    
    public static function load(string $file): string {
        $content = file_get_contents($file);
        
        if (!self::isEncrypted($content)) {
            return $content;
        }
        
        return self::decrypt($content);
    }
    
    private static function isEncrypted(string $data): bool {
        for ($i = 0; $i < count(self::HEADER); $i++) {
            if (ord($data[$i]) !== self::HEADER[$i]) {
                return false;
            }
        }
        return true;
    }
    
    private static function decrypt(string $data): string {
        $headerLen = count(self::HEADER);
        $data = substr($data, $headerLen);
        $key = self::KEY;
        $keyLen = count($key);
        $p = 0;
        
        for ($i = 0; $i < strlen($data); $i++) {
            if ($i & 1) {
                $p = ($p + $key[$p] + $i) % $keyLen;
                $data[$i] = chr(~(ord($data[$i]) ^ $key[$p]));
            }
        }
        
        return $data;
    }
}

// 自动加载器
spl_autoload_register(function ($class) {
    $file = str_replace('\\', '/', $class) . '.php';
    $encrypted = $file . '.enc';
    
    if (file_exists($encrypted)) {
        $code = PHPGuardLoader::load($encrypted);
        eval('?>' . $code);
    }
});
```

## 总结

| 方案 | 难度 | 可用性 | 推荐度 |
|------|------|--------|--------|
| WSL 构建 | 低 | 立即可用 | ⭐⭐⭐⭐⭐ |
| Docker 构建 | 低 | CI/CD 友好 | ⭐⭐⭐⭐ |
| GitHub Actions | 中 | 自动发布 | ⭐⭐⭐⭐ |
| Windows DLL | 高 | 需适配 | ⭐⭐ |
| 纯 PHP | 中 | 降级方案 | ⭐⭐⭐ |

**当前建议：** 专注于 WSL/Docker/GitHub Actions 方案，提供 Linux/macOS 预编译版本。Windows 用户可以通过 WSL 运行或使用纯 PHP 备选方案。
