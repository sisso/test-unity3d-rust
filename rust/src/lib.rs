#[macro_use]
pub mod debug;
pub mod schema_generated;
pub mod test_ffi;

use crate::schema_generated::messages;
use flatbuffers::FlatBufferBuilder;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::test_ffi::*;

fn to_cstr(value: &str) -> *mut c_char {
    CString::new(value).unwrap().into_raw()
}

fn from_cstr(ptr: *const c_char) -> String {
    let c_str = unsafe {
        assert!(!ptr.is_null());
        CStr::from_ptr(ptr)
    };

    c_str.to_str().unwrap().to_string()
}

fn to_slice<'a, T>(buffer: *const T, length: u32) -> &'a [T] {
    unsafe { std::slice::from_raw_parts(buffer, length as usize) }
}

#[no_mangle]
pub extern "C" fn test_ffi_context_create() -> *mut FfiContext {
    let context = FfiContext::new();

    debug!("context_create {:?}", context);

    Box::into_raw(Box::new(context))
}

#[no_mangle]
pub extern "C" fn test_ffi_context_close(ctx_ptr: *mut FfiContext) {
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
    ctx_ptr: *mut FfiContext,
    value: *const c_char,
) -> bool {
    let c_str = unsafe {
        assert!(!value.is_null());
        CStr::from_ptr(value)
    };

    let value = c_str.to_str().unwrap();
    let ctx = FfiContext::from_ptr(ctx_ptr);
    debug!(
        "context_set_string {:?}: {}",
        ctx.get_control_value(),
        value
    );
    ctx.string = Some(value.to_string());
    true
}

#[no_mangle]
pub extern "C" fn test_ffi_context_get_string(ctx_ptr: *mut FfiContext) -> *mut c_char {
    let ctx = FfiContext::from_ptr(ctx_ptr);

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
pub extern "C" fn test_ffi_context_set_struct(ctx_ptr: *mut FfiContext, value: V2) -> bool {
    let ctx = FfiContext::from_ptr(ctx_ptr);
    debug!(
        "context_set_struct {:?}: {:?}",
        ctx.get_control_value(),
        value
    );
    ctx.v2 = Some(value);
    true
}

#[no_mangle]
pub extern "C" fn test_ffi_context_get_struct(ctx_ptr: *mut FfiContext) -> V2 {
    let ctx = FfiContext::from_ptr(ctx_ptr);
    let value = ctx.v2.take().unwrap_or(V2 { x: 0, y: 0 });
    debug!(
        "context_get_struct {:?}: {:?}",
        ctx.get_control_value(),
        value
    );
    value
}

#[no_mangle]
pub extern "C" fn test_ffi_context_set_array(
    ctx_ptr: *mut FfiContext,
    buffer: *mut u8,
    length: u32,
) -> bool {
    let ctx = FfiContext::from_ptr(ctx_ptr);
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
    ctx_ptr: *mut FfiContext,
    callback: extern "stdcall" fn(*mut u8, u32),
) -> bool {
    let ctx = FfiContext::from_ptr(ctx_ptr);

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
    ctx_ptr: *mut FfiContext,
    buffer: *mut V2,
    length: u32,
) -> bool {
    let ctx = FfiContext::from_ptr(ctx_ptr);
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
    ctx_ptr: *mut FfiContext,
    callback: extern "stdcall" fn(*mut V2, u32),
) -> bool {
    let ctx = FfiContext::from_ptr(ctx_ptr);

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
    ctx_ptr: *mut FfiContext,
    buffer: *const FFIPerson,
    length: u32,
) -> bool {
    let ctx = FfiContext::from_ptr(ctx_ptr);
    let ref_data = to_slice(buffer, length);
    debug!(
        "context_set_persons {:?}: {:?}",
        ctx.get_control_value(),
        ref_data
    );

    let people = ref_data
        .iter()
        .map(|ffi_person| Person {
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
    ctx_ptr: *mut FfiContext,
    callback: extern "stdcall" fn(*mut FFIPerson, u32),
) -> bool {
    let ctx = FfiContext::from_ptr(ctx_ptr);

    let people = &ctx.people;
    let len = people.len() as u32;

    let mut people_ffi = people
        .into_iter()
        .map(|person| FFIPerson {
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
    ctx_ptr: *mut FfiContext,
    buffer: *const u8,
    length: u32,
) -> bool {
    let ctx = FfiContext::from_ptr(ctx_ptr);
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
    ctx_ptr: *mut FfiContext,
    callback: extern "stdcall" fn(*const u8, u32),
) -> bool {
    let ctx = FfiContext::from_ptr(ctx_ptr);

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
