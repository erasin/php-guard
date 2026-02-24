use std::os::raw::c_int;
use std::ptr;

use std::os::unix::io::AsRawFd;

use phper::strings::ZStr;
use phper::sys::{self, zend_compile_file, zend_file_handle};

use crate::config::HEADER;
use crate::crypto::decode;
use crate::file_handler::create_temp_file_with_content;

static mut ORIGINAL_COMPILE_FILE: Option<
    unsafe extern "C" fn(*mut zend_file_handle, c_int) -> *mut sys::_zend_op_array,
> = None;

pub unsafe fn init_hooks() {
    ORIGINAL_COMPILE_FILE = Some(zend_compile_file.unwrap());
}

pub unsafe fn restore_hooks() {
    if let Some(original) = ORIGINAL_COMPILE_FILE {
        sys::zend_compile_file = Some(original);
    }
}

unsafe fn call_original(
    file_handle: *mut zend_file_handle,
    type_: c_int,
) -> *mut sys::_zend_op_array {
    match ORIGINAL_COMPILE_FILE {
        Some(original) => original(file_handle, type_),
        None => ptr::null_mut(),
    }
}

unsafe fn get_filename_str(handle: &zend_file_handle) -> Option<String> {
    if handle.filename.is_null() {
        return None;
    }

    let zstr = ZStr::from_ptr(handle.filename);
    Some(zstr.to_string_lossy().into_owned())
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

fn try_decrypt(filename: &str) -> Option<std::fs::File> {
    let content = std::fs::read(filename).ok()?;

    if content.len() < HEADER.len() {
        return None;
    }

    if &content[..HEADER.len()] != HEADER {
        return None;
    }

    let mut decrypted = content[HEADER.len()..].to_vec();
    decode(&mut decrypted);

    create_temp_file_with_content(&decrypted).ok()
}

#[no_mangle]
pub unsafe extern "C" fn php_guard_compile_file(
    file_handle: *mut zend_file_handle,
    type_: c_int,
) -> *mut sys::_zend_op_array {
    if file_handle.is_null() {
        return call_original(file_handle, type_);
    }

    let handle = &mut *file_handle;

    let filename = match get_filename_str(handle) {
        Some(s) => s,
        None => return call_original(file_handle, type_),
    };

    if !should_decrypt(&filename) {
        return call_original(file_handle, type_);
    }

    let temp_file = match try_decrypt(&filename) {
        Some(f) => f,
        None => return call_original(file_handle, type_),
    };

    if !handle.handle.fp.is_null() {
        libc::fclose(handle.handle.fp.cast());
    }

    let fd = temp_file.as_raw_fd();
    let new_fp = libc::fdopen(fd, b"r\0".as_ptr().cast());
    handle.handle.fp = new_fp.cast();

    std::mem::forget(temp_file);

    call_original(file_handle, type_)
}

pub unsafe fn register_hooks() {
    sys::zend_compile_file = Some(php_guard_compile_file);
}
