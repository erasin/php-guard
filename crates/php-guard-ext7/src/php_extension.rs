use crate::hooks;

use phper::{functions::Argument, modules::Module, php_get_module, strings::ZString, values::ZVal};

use php_guard_core::{crypto, file_handler};

const MODULE_NAME: &str = env!("CARGO_PKG_NAME");
const MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");
const MODULE_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn php_guard_encode(arguments: &mut [ZVal]) -> phper::Result<Option<ZString>> {
    let content = arguments[0].expect_z_str()?;
    let content_bytes = content.to_bytes();

    if crypto::is_encrypted(content_bytes) {
        return Ok(None);
    }

    let encrypted = file_handler::encrypt_content(content_bytes);
    Ok(Some(ZString::new(&encrypted)))
}

fn php_guard_is_encrypted(arguments: &mut [ZVal]) -> phper::Result<bool> {
    let content = arguments[0].expect_z_str()?;
    let content_bytes = content.to_bytes();
    Ok(crypto::is_encrypted(content_bytes))
}

fn php_guard_version(_arguments: &mut [ZVal]) -> phper::Result<&'static str> {
    Ok(MODULE_VERSION)
}

#[php_get_module]
pub fn get_module() -> Module {
    register_module()
}

pub fn register_module() -> Module {
    let mut module = Module::new(MODULE_NAME, MODULE_VERSION, MODULE_AUTHORS);

    module
        .add_function("php_guard_encode", php_guard_encode)
        .argument(Argument::new("content"));

    module
        .add_function("php_guard_is_encrypted", php_guard_is_encrypted)
        .argument(Argument::new("content"));

    module.add_function("php_guard_version", php_guard_version);

    unsafe {
        hooks::init_hooks();
        hooks::register_hooks();
    }

    module
}

#[unsafe(no_mangle)]
pub extern "C" fn php_guard_mshutdown(_type: i32, _module_number: i32) -> i32 {
    unsafe {
        hooks::restore_hooks();
    }
    0
}
