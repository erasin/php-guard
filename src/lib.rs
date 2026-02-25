pub mod config;
pub mod crypto;
pub mod file_handler;

#[cfg(all(unix, feature = "php-extension"))]
mod hooks;

#[cfg(all(unix, feature = "php-extension"))]
mod php_extension;

pub use config::{HEADER, KEY};
pub use crypto::{decode, is_encrypted};
pub use file_handler::{
    create_temp_file_with_content, encrypt_content, encrypt_file, read_and_decrypt_file,
};

#[cfg(all(unix, feature = "php-extension"))]
pub use php_extension::register_module;
