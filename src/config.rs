// 配置文件由 build.rs 自动生成
// 运行 scripts/generate-key.sh 或 scripts/generate-key.bat 生成配置

include!(concat!(env!("OUT_DIR"), "/config_generated.rs"));
