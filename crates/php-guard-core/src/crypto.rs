use crate::config::{HEADER, KEY};

pub fn encode(data: &mut [u8]) {
    let key_len = KEY.len();
    let mut p: usize = 0;

    for (i, byte) in data.iter_mut().enumerate() {
        if i & 1 == 1 {
            p = p.wrapping_add(KEY[p] as usize).wrapping_add(i);
            p %= key_len;
            let t = KEY[p];
            *byte = !(*byte ^ t);
        }
    }
}

pub fn decode(data: &mut [u8]) {
    let key_len = KEY.len();
    let mut p: usize = 0;

    for (i, byte) in data.iter_mut().enumerate() {
        if i & 1 == 1 {
            p = p.wrapping_add(KEY[p] as usize).wrapping_add(i);
            p %= key_len;
            let t = KEY[p];
            *byte = !(*byte) ^ t;
        }
    }
}

pub fn is_encrypted(data: &[u8]) -> bool {
    if data.len() < HEADER.len() {
        return false;
    }
    &data[..HEADER.len()] == HEADER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let original = b"<?php echo 'Hello, World!'; ?>".to_vec();
        let mut data = original.clone();

        encode(&mut data);
        assert!(!is_encrypted(&data));
        assert_ne!(data, original);

        decode(&mut data);
        assert_eq!(data, original);
    }

    #[test]
    fn test_is_encrypted() {
        let encrypted_with_header = [HEADER, b"test"].concat();
        assert!(is_encrypted(&encrypted_with_header));

        let plain = b"<?php echo 'test';";
        assert!(!is_encrypted(plain));
    }
}
