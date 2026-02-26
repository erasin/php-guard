pub mod config;
pub mod crypto;
pub mod file_handler;

pub use config::{HEADER, KEY};
pub use crypto::{decode, encode, is_encrypted};
pub use file_handler::{
    check_file_encrypted, create_temp_file_with_content, encrypt_content, encrypt_file,
    read_and_decrypt_file,
};
