use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::config::HEADER;
use crate::crypto::{decode, is_encrypted};

pub fn read_and_decrypt_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len() as usize;

    if file_size < HEADER.len() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "File too small to be encrypted",
        ));
    }

    let mut header_buf = vec![0u8; HEADER.len()];
    file.read_exact(&mut header_buf)?;

    if header_buf.as_slice() != HEADER {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "File is not encrypted or has wrong header",
        ));
    }

    let data_len = file_size - HEADER.len();
    let mut data = vec![0u8; data_len];
    if data_len > 0 {
        file.read_exact(&mut data)?;
    }

    decode(&mut data);

    Ok(data)
}

pub fn encrypt_content(content: &[u8]) -> Vec<u8> {
    if is_encrypted(content) {
        return content.to_vec();
    }

    let mut result = Vec::with_capacity(HEADER.len() + content.len());
    result.extend_from_slice(HEADER);

    let mut encrypted_content = content.to_vec();
    crate::crypto::encode(&mut encrypted_content);
    result.extend_from_slice(&encrypted_content);

    result
}

pub fn encrypt_file<P: AsRef<Path>, Q: AsRef<Path>>(source: P, dest: Q) -> std::io::Result<()> {
    let mut content = Vec::new();
    File::open(source)?.read_to_end(&mut content)?;

    if is_encrypted(&content) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "File is already encrypted",
        ));
    }

    let encrypted = encrypt_content(&content);
    let mut dest_file = File::create(dest)?;
    dest_file.write_all(&encrypted)?;

    Ok(())
}

pub fn check_file_encrypted<P: AsRef<Path>>(path: P) -> std::io::Result<bool> {
    let mut file = File::open(path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len() as usize;

    if file_size < HEADER.len() {
        return Ok(false);
    }

    let mut header_buf = vec![0u8; HEADER.len()];
    file.read_exact(&mut header_buf)?;

    Ok(header_buf.as_slice() == HEADER)
}

pub fn create_temp_file_with_content(content: &[u8]) -> std::io::Result<File> {
    let mut temp_file = tempfile::tempfile()?;
    temp_file.write_all(content)?;
    temp_file.seek(SeekFrom::Start(0))?;
    Ok(temp_file)
}
