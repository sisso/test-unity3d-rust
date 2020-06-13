#[macro_use]
pub mod debug;
pub mod ffi_utils;
pub mod schema_generated;
pub mod server;
pub mod server_ffi;
pub mod test_ffi;

use crate::schema_generated::messages;
use crate::server::{RawMsg, Server};
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
    namespace: u16,
    kind: u16,
    buffer: *mut u8,
    length: u32,
) -> bool {
    let ctx = server_ffi::FfiContext::from_ptr(ctx_ptr);
    let ref_data = to_slice(buffer, length);
    // debug!("server_ffi_push {:?}: {:?}", ctx.get_control_value(), value);
    match ctx.push(
        FFI_USER_ID,
        &RawMsg {
            namespace,
            kind,
            body: ref_data,
        },
    ) {
        Err(err) => {
            debug!(
                "server_ffi_push fail for ({:?}, {:?}, {:?}): {:?}",
                namespace, kind, ref_data, err
            );
            false
        }
        _ => true,
    }
}

#[no_mangle]
pub extern "C" fn server_ffi_take(
    ctx_ptr: *mut server_ffi::FfiContext,
    callback: extern "stdcall" fn(u16, u16, *const u8, u32),
) -> bool {
    let ctx = server_ffi::FfiContext::from_ptr(ctx_ptr);

    match ctx.take(FFI_USER_ID) {
        Ok(messages) => {
            for msg in messages {
                callback(
                    msg.namespace,
                    msg.kind,
                    msg.body.as_ptr(),
                    msg.body.len() as u32,
                );
            }
            true
        }

        Err(err) => {
            debug!("server_ffi_take fail: {:?}", err);
            false
        }
    }
}

// -------------------------------------------------------------------------------------------------
// test_ffi
// -------------------------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn test_ffi_context_create() -> *mut test_ffi::FfiContext {
    let context = test_ffi::FfiContext::new();

    debug!("context_create {:?}", context);

    Box::into_raw(Box::new(context))
}

#[no_mangle]
pub extern "C" fn test_ffi_context_close(ctx_ptr: *mut test_ffi::FfiContext) {
    if ctx_ptr.is_null() {
        return;
    }
    let ctx = unsafe {
        Box::from_raw(ctx_ptr);
    };
    debug!("context_close {:?}", ctx);
}

#[no_mangle]
pub extern "C" fn test_ffi_context_set_string(
    ctx_ptr: *mut test_ffi::FfiContext,
    value: *const c_char,
) -> bool {
    let c_str = unsafe {
        assert!(!value.is_null());
        CStr::from_ptr(value)
    };

    let value = c_str.to_str().unwrap();
    let ctx = test_ffi::FfiContext::from_ptr(ctx_ptr);
    debug!(
        "context_set_string {:?}: {}",
        ctx.get_control_value(),
        value
    );
    ctx.string = Some(value.to_string());
    true
}

#[no_mangle]
pub extern "C" fn test_ffi_context_get_string(ctx_ptr: *mut test_ffi::FfiContext) -> *mut c_char {
    let ctx = test_ffi::FfiContext::from_ptr(ctx_ptr);

    let value = ctx.string.take();

    debug!(
        "context_get_string {:?}: {:?}",
        ctx.get_control_value(),
        value
    );

    match value {
        Some(string) => {
            let c_str_song = CString::new(string).unwrap();
            c_str_song.into_raw()
        }
        _ => {
            let c_str_song = CString::new("").unwrap();
            c_str_song.into_raw()
        }
    }
}

#[no_mangle]
pub extern "C" fn test_ffi_context_set_struct(
    ctx_ptr: *mut test_ffi::FfiContext,
    value: test_ffi::V2,
) -> bool {
    let ctx = test_ffi::FfiContext::from_ptr(ctx_ptr);
    debug!(
        "context_set_struct {:?}: {:?}",
        ctx.get_control_value(),
        value
    );
    ctx.v2 = Some(value);
    true
}

#[no_mangle]
pub extern "C" fn test_ffi_context_get_struct(ctx_ptr: *mut test_ffi::FfiContext) -> test_ffi::V2 {
    let ctx = test_ffi::FfiContext::from_ptr(ctx_ptr);
    let value = ctx.v2.take().unwrap_or(test_ffi::V2 { x: 0, y: 0 });
    debug!(
        "context_get_struct {:?}: {:?}",
        ctx.get_control_value(),
        value
    );
    value
}

#[no_mangle]
pub extern "C" fn test_ffi_context_set_array(
    ctx_ptr: *mut test_ffi::FfiContext,
    buffer: *mut u8,
    length: u32,
) -> bool {
    let ctx = test_ffi::FfiContext::from_ptr(ctx_ptr);
    let ref_data = unsafe { std::slice::from_raw_parts(buffer, length as usize) };
    let value = ref_data.to_vec();
    debug!(
        "context_set_array {:?}: {:?}",
        ctx.get_control_value(),
        value
    );
    ctx.array = Some(value);
    true
}

#[no_mangle]
pub extern "C" fn test_ffi_context_get_array(
    ctx_ptr: *mut test_ffi::FfiContext,
    callback: extern "stdcall" fn(*mut u8, u32),
) -> bool {
    let ctx = test_ffi::FfiContext::from_ptr(ctx_ptr);

    match ctx.array.take() {
        Some(mut value) => {
            debug!(
                "context_get_array {:?}: {:?}",
                ctx.get_control_value(),
                value
            );
            callback(value.as_mut_ptr(), value.len() as u32);
            true
        }
        None => false,
    }
}

#[no_mangle]
pub extern "C" fn test_ffi_context_set_struct_array(
    ctx_ptr: *mut test_ffi::FfiContext,
    buffer: *mut test_ffi::V2,
    length: u32,
) -> bool {
    let ctx = test_ffi::FfiContext::from_ptr(ctx_ptr);
    let ref_data = to_slice(buffer, length);
    let value = ref_data.to_vec();
    debug!(
        "context_set_struct_array {:?}: {:?}",
        ctx.get_control_value(),
        value
    );
    ctx.v2_array = Some(value);
    true
}

#[no_mangle]
pub extern "C" fn test_ffi_context_get_struct_array(
    ctx_ptr: *mut test_ffi::FfiContext,
    callback: extern "stdcall" fn(*mut test_ffi::V2, u32),
) -> bool {
    let ctx = test_ffi::FfiContext::from_ptr(ctx_ptr);

    match ctx.v2_array.take() {
        Some(mut value) => {
            debug!(
                "context_get_struct_array {:?}: {:?}",
                ctx.get_control_value(),
                value
            );
            callback(value.as_mut_ptr(), value.len() as u32);
            true
        }
        None => false,
    }
}

#[no_mangle]
pub extern "C" fn test_ffi_context_set_people(
    ctx_ptr: *mut test_ffi::FfiContext,
    buffer: *const test_ffi::FFIPerson,
    length: u32,
) -> bool {
    let ctx = test_ffi::FfiContext::from_ptr(ctx_ptr);
    let ref_data = to_slice(buffer, length);
    debug!(
        "context_set_persons {:?}: {:?}",
        ctx.get_control_value(),
        ref_data
    );

    let people = ref_data
        .iter()
        .map(|ffi_person| test_ffi::Person {
            id: ffi_person.id,
            name: from_cstr(ffi_person.name),
            number: ffi_person.number,
        })
        .collect();

    ctx.people = people;
    true
}

#[no_mangle]
pub extern "C" fn test_ffi_context_get_people(
    ctx_ptr: *mut test_ffi::FfiContext,
    callback: extern "stdcall" fn(*mut test_ffi::FFIPerson, u32),
) -> bool {
    let ctx = test_ffi::FfiContext::from_ptr(ctx_ptr);

    let people = &ctx.people;
    let len = people.len() as u32;

    let mut people_ffi = people
        .into_iter()
        .map(|person| test_ffi::FFIPerson {
            id: person.id,
            name: to_cstr(person.name.as_str()),
            number: person.number,
        })
        .collect::<Vec<_>>();

    callback(people_ffi.as_mut_ptr(), len);

    true
}

#[no_mangle]
pub extern "C" fn test_ffi_context_set_flatbuffer(
    ctx_ptr: *mut test_ffi::FfiContext,
    buffer: *const u8,
    length: u32,
) -> bool {
    let ctx = test_ffi::FfiContext::from_ptr(ctx_ptr);
    let ref_data = unsafe { std::slice::from_raw_parts(buffer, length as usize) };

    let root = messages::get_root_as_messages(ref_data);
    debug!(
        "context_set_array {:?}: {:?}",
        ctx.get_control_value(),
        ref_data
    );

    if let Some(input) = root.input() {
        ctx.vectors.clear();
        for v in input {
            ctx.vectors.push((v.x(), v.y()));
        }
    }

    true
}

#[no_mangle]
pub extern "C" fn test_ffi_context_get_flatbuffer(
    ctx_ptr: *mut test_ffi::FfiContext,
    callback: extern "stdcall" fn(*const u8, u32),
) -> bool {
    let ctx = test_ffi::FfiContext::from_ptr(ctx_ptr);

    let mut b = FlatBufferBuilder::new();

    let vecs: Vec<messages::V2> = ctx
        .vectors
        .iter()
        .map(|(x, y)| messages::V2::new(*x, *y))
        .collect();
    let vectors = b.create_vector(vecs.as_ref());

    let args = messages::MessagesArgs {
        input: None,
        output: Some(vectors),
    };

    let root = messages::Messages::create(&mut b, &args);
    b.finish_minimal(root);

    let raw_data = b.finished_data();
    debug!(
        "context_get_flatbuffer {:?}: {:?}",
        ctx.get_control_value(),
        raw_data
    );

    callback(raw_data.as_ptr(), raw_data.len() as u32);

    true
}

#[no_mangle]
pub extern "C" fn test_ffi_free_string(ptr: *mut c_char) -> bool {
    assert!(!ptr.is_null());
    unsafe {
        CString::from_raw(ptr);
    };
    true
}

#[no_mangle]
pub extern "C" fn test_ffi_add_numbers(number1: i32, number2: i32) -> i32 {
    number1 + number2
}
// -------------------------------------------------------------------------------------------------
