mod config;
mod crypto;
mod file_handler;

#[cfg(all(unix, feature = "php-extension"))]
mod hooks;

#[cfg(all(unix, feature = "php-extension"))]
mod php_extension;

pub use crypto::{decode, encode, is_encrypted};
pub use file_handler::{
    check_file_encrypted, create_temp_file_with_content, encrypt_content, encrypt_file,
    read_and_decrypt_file,
};

#[cfg(all(unix, feature = "php-extension"))]
pub use php_extension::register_module;
