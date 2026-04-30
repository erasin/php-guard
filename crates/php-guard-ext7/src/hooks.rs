use std::ffi::CStr;
use std::os::raw::c_int;
use std::os::unix::io::AsRawFd;
use std::ptr;

use phper::sys::{
    self, zend_compile_file, zend_file_handle, zend_stream_type_ZEND_HANDLE_FILENAME,
    zend_stream_type_ZEND_HANDLE_FP, zend_stream_type_ZEND_HANDLE_STREAM,
};

use php_guard_core::config::HEADER;
use php_guard_core::crypto::decode;

static mut ORIGINAL_COMPILE_FILE: Option<
    unsafe extern "C" fn(*mut zend_file_handle, c_int) -> *mut sys::_zend_op_array,
> = None;

pub unsafe fn init_hooks() {
    unsafe {
        ORIGINAL_COMPILE_FILE = Some(zend_compile_file.unwrap());
    }
}

pub unsafe fn restore_hooks() {
    unsafe {
        if let Some(original) = ORIGINAL_COMPILE_FILE {
            sys::zend_compile_file = Some(original);
        }
    }
}

unsafe fn call_original(
    file_handle: *mut zend_file_handle,
    type_: c_int,
) -> *mut sys::_zend_op_array {
    unsafe {
        match ORIGINAL_COMPILE_FILE {
            Some(original) => original(file_handle, type_),
            None => ptr::null_mut(),
        }
    }
}

unsafe fn get_filename_str(handle: &zend_file_handle) -> Option<String> {
    unsafe {
        if handle.filename.is_null() {
            return None;
        }
        match CStr::from_ptr(handle.filename).to_str() {
            Ok(s) => Some(s.to_string()),
            Err(_) => None,
        }
    }
}

fn should_decrypt(filename: &str) -> bool {
    if filename == "-" {
        return false;
    }
    if filename.starts_with("phar:") {
        return false;
    }
    true
}

fn try_decrypt(filename: &str) -> Option<Vec<u8>> {
    let content = std::fs::read(filename).ok()?;

    if content.len() < HEADER.len() {
        return None;
    }

    if &content[..HEADER.len()] != HEADER {
        return None;
    }

    let mut decrypted = content[HEADER.len()..].to_vec();
    decode(&mut decrypted);

    Some(decrypted)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn php_guard_compile_file(
    file_handle: *mut zend_file_handle,
    type_: c_int,
) -> *mut sys::_zend_op_array {
    if file_handle.is_null() {
        return unsafe { call_original(file_handle, type_) };
    }

    let handle = unsafe { &mut *file_handle };

    let filename = match unsafe { get_filename_str(handle) } {
        Some(s) => s,
        None => return unsafe { call_original(file_handle, type_) },
    };

    if !should_decrypt(&filename) {
        return unsafe { call_original(file_handle, type_) };
    }

    let decrypted = match try_decrypt(&filename) {
        Some(d) => d,
        None => return unsafe { call_original(file_handle, type_) },
    };

    let mut temp_file = match tempfile::tempfile() {
        Ok(f) => f,
        Err(_) => return unsafe { call_original(file_handle, type_) },
    };

    use std::io::{Seek, SeekFrom, Write};
    if temp_file.write_all(&decrypted).is_err() {
        return unsafe { call_original(file_handle, type_) };
    }
    if temp_file.seek(SeekFrom::Start(0)).is_err() {
        return unsafe { call_original(file_handle, type_) };
    }

    match handle.type_ {
        zend_stream_type_ZEND_HANDLE_FP => unsafe {
            if !handle.handle.fp.is_null() {
                libc::fclose(handle.handle.fp.cast());
                handle.handle.fp = ptr::null_mut();
            }
        },
        zend_stream_type_ZEND_HANDLE_STREAM => unsafe {
            if !handle.handle.stream.handle.is_null() {
                if let Some(closer) = handle.handle.stream.closer {
                    closer(handle.handle.stream.handle);
                }
            }
            handle.handle.stream.handle = ptr::null_mut();
        },
        zend_stream_type_ZEND_HANDLE_FILENAME => {}
        _ => {}
    }

    let fd = temp_file.as_raw_fd();
    let new_fp = unsafe { libc::fdopen(fd, b"r\0".as_ptr().cast()) };
    if new_fp.is_null() {
        return unsafe { call_original(file_handle, type_) };
    }

    unsafe {
        handle.handle.fp = new_fp.cast();
    }
    handle.type_ = zend_stream_type_ZEND_HANDLE_FP;
    std::mem::forget(temp_file);

    unsafe { call_original(file_handle, type_) }
}

pub unsafe fn register_hooks() {
    unsafe {
        sys::zend_compile_file = Some(php_guard_compile_file);
    }
}
