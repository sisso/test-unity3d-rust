extern crate game;

use crate::ffi::ffi_utils::*;
use crate::ffi::*;

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
pub extern "C" fn ffi_context_create_and_connect() -> Box<FfiContext> {
    let address = "localhost:28483";
    let context = FfiContext::new(Some(address));
    debug!("context_create {:?} to {:?}", context, address);
    Box::new(context)
}

#[no_mangle]
pub extern "C" fn ffi_context_close(ctx: Box<FfiContext>) {
    debug!("context_close {:?}", ctx);
}

pub extern "C" fn ffi_context_push(ctx: &mut FfiContext, buffer: *mut u8, length: u32) -> bool {
    let ref_data = to_slice(buffer, length);
    // debug!("server_ffi_push {:?}: {:?}", ctx.get_control_value(), value);
    match ctx.push(ref_data) {
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
    callback: extern "stdcall" fn(*const u8, u32),
) -> bool {
    // debug!("server_ffi_push {:?}: {:?}", ctx.get_control_value(), value);
    match ctx.take() {
        Ok(buffer) => {
            callback(buffer.as_ptr(), buffer.len() as u32);
            true
        }

        Err(err) => {
            debug!("server_ffi_take fail: {:?}", err);
            false
        }
    }
}
