#[macro_use]
pub mod debug;
pub mod ffi_utils;
pub mod schemas;
pub mod server;

use crate::server::{server_ffi, RawMsg, ServerConnector};

use ffi_utils::*;
use flatbuffers::FlatBufferBuilder;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// -------------------------------------------------------------------------------------------------
// server_ffi
// -------------------------------------------------------------------------------------------------

const FFI_USER_ID: u16 = 0;

#[no_mangle]
pub extern "C" fn server_ffi_context_create() -> *mut server_ffi::FfiContext {
    let context = server_ffi::FfiContext::new();

    debug!("context_create {:?}", context);

    Box::into_raw(Box::new(context))
}

#[no_mangle]
pub extern "C" fn server_ffi_context_close(ctx_ptr: *mut server_ffi::FfiContext) {
    if ctx_ptr.is_null() {
        return;
    }
    let ctx = unsafe {
        Box::from_raw(ctx_ptr);
    };
    debug!("context_close {:?}", ctx);
}

pub extern "C" fn server_ffi_push(
    ctx_ptr: *mut server_ffi::FfiContext,
    buffer: *mut u8,
    length: u32,
) -> bool {
    let ctx = server_ffi::FfiContext::from_ptr(ctx_ptr);
    let ref_data = to_slice(buffer, length);
    // debug!("server_ffi_push {:?}: {:?}", ctx.get_control_value(), value);
    match ctx.push(FFI_USER_ID, ref_data) {
        Err(err) => {
            debug!("server_ffi_push fail: {:?}", err);
            false
        }
        _ => true,
    }
}

#[no_mangle]
pub extern "C" fn server_ffi_take(
    ctx_ptr: *mut server_ffi::FfiContext,
    callback: extern "stdcall" fn(*const u8, u32),
) -> bool {
    let ctx = server_ffi::FfiContext::from_ptr(ctx_ptr);
    // debug!("server_ffi_push {:?}: {:?}", ctx.get_control_value(), value);
    match ctx.take(FFI_USER_ID) {
        Ok(messages) => {
            for msg in messages {
                callback(msg.as_ptr(), msg.len() as u32);
            }
            true
        }

        Err(err) => {
            debug!("server_ffi_take fail: {:?}", err);
            false
        }
    }
}
