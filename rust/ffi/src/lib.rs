extern crate game;

use crate::ffi::ffi_utils::*;
use crate::ffi::*;
use game::Error;
use std::os::raw::c_char;

#[macro_use]
pub mod debug;
mod client;
pub mod ffi;

#[no_mangle]
pub extern "C" fn ffi_context_create_embedded() -> Box<FfiContext> {
    let context = FfiContext::new(None);
    debug!("context_create {:?}", context);
    Box::new(context)
}

#[no_mangle]
pub extern "C" fn ffi_context_connect(address: *const c_char) -> Box<FfiContext> {
    let address = "localhost:3333";
    let context = FfiContext::new(Some(address));
    debug!("context_create {:?} to {:?}", context, address);
    Box::new(context)
}

#[no_mangle]
pub extern "C" fn ffi_context_close(ctx: Box<FfiContext>) {
    debug!("context_close {:?}", ctx);
}

#[no_mangle]
pub extern "C" fn ffi_context_push(
    ctx: &mut FfiContext,
    package_kind: u16,
    buffer: *mut u8,
    length: u32,
) -> bool {
    let ref_data = to_slice(buffer, length);
    debug!("ffi_context_push {:?}", package_kind);
    match ctx.push(package_kind, ref_data) {
        Err(err) => {
            debug!("server_ffi_push fail: {:?}", err);
            false
        }
        _ => true,
    }
}

#[no_mangle]
pub extern "C" fn ffi_context_take(
    ctx: &mut FfiContext,
    callback: extern "stdcall" fn(u16, *const u8, u32),
) -> bool {
    // debug!("server_ffi_push {:?}: {:?}", ctx.get_control_value(), value);
    match ctx.take() {
        Ok(Some((kind, buffer))) => {
            callback(kind, buffer.as_ptr(), buffer.len() as u32);
            true
        }

        Ok(None) => true,

        Err(Error::Disconnect) => false,

        Err(err) => {
            debug!("server_ffi_take fail: {:?}", err);
            false
        }
    }
}
