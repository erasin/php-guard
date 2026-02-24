use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardConfig {
    #[serde(with = "serde_bytes_vec")]
    pub header: Vec<u8>,
    #[serde(with = "serde_bytes_vec")]
    pub key: Vec<u8>,
}

impl GuardConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(&path)?;
        let config: ConfigFile = toml::from_str(&content)?;
        Ok(config.guard)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let config = ConfigFile {
            guard: self.clone(),
        };
        let content = toml::to_string_pretty(&config)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn generate(header_len: usize, key_len: usize) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let header: Vec<u8> = (0..header_len).map(|_| rng.gen()).collect();
        let key: Vec<u8> = (0..key_len).map(|_| rng.gen()).collect();

        Self { header, key }
    }

    pub fn to_rust_code(&self) -> String {
        let header_str = format_bytes_rust(&self.header);
        let key_str = format_bytes_rust(&self.key);

        format!(
            "pub const HEADER: &[u8] = &[\n{}];\n\npub const KEY: &[u8] = &[\n{}];\n",
            header_str, key_str
        )
    }

    pub fn to_php_code(&self) -> String {
        let header_str = format_bytes_php(&self.header);
        let key_str = format_bytes_php(&self.key);

        format!(
            "const HEADER = [\n{}];\n\nconst KEY = [\n{}];\n",
            header_str, key_str
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    guard: GuardConfig,
}

mod serde_bytes_vec {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex_vec: Vec<String> = bytes.iter().map(|b| format!("0x{:02x}", b)).collect();
        hex_vec.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_vec: Vec<String> = Vec::deserialize(deserializer)?;
        hex_vec
            .iter()
            .map(|s| {
                let s = s.trim_start_matches("0x").trim_start_matches("0X");
                u8::from_str_radix(s, 16).map_err(serde::de::Error::custom)
            })
            .collect()
    }
}

fn format_bytes_rust(bytes: &[u8]) -> String {
    let chunks: Vec<String> = bytes
        .chunks(4)
        .map(|chunk| {
            chunk
                .iter()
                .map(|b| format!("0x{:02x}", b))
                .collect::<Vec<_>>()
                .join(", ")
                + ","
        })
        .collect();

    format!("    {}", chunks.join("\n    "))
}

fn format_bytes_php(bytes: &[u8]) -> String {
    let chunks: Vec<String> = bytes
        .chunks(4)
        .map(|chunk| {
            chunk
                .iter()
                .map(|b| format!("0x{:02x}", b))
                .collect::<Vec<_>>()
                .join(", ")
                + ","
        })
        .collect();

    format!("    {}", chunks.join("\n    "))
}
