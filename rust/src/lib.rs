#[macro_use]
mod debug;

use std::ffi::CString;
use std::ffi::CStr;
use std::mem::{drop, transmute};
use std::net::UdpSocket;
use std::os::raw::c_char;
use std::ptr;
use std::result::Result;
use std::io::Write;

#[repr(C)]
pub struct ByteBuffer {
    len: i32,
    data: *mut u8,
}

enum Requests {
    SetValue,
    GetValue,
}

enum Responses {
    Value {
        result: u32,
    }
}

#[derive(Debug)]
pub struct Context {
    input: i32,
    output: u32,
    requests: Option<Vec<String>>,
    responses: Option<Vec<String>>,
    byte_requests: Option<Vec<u8>>,
    byte_responses: Option<Vec<u8>>,
    string: Option<String>,
    v2: Option<V2>,
}

impl<'a> Context {
    fn new() -> Self {
        Context {
            input: 0,
            output: 0,
            requests: None,
            responses: None,
            byte_requests: None,
            byte_responses: None,
            string: None,
            v2: None
        }
    }

    fn from_ptr(ptr: *mut Context) -> &'a mut Context {
        assert!(!ptr.is_null());

        unsafe {
            &mut *ptr
        }
    }

    fn set_input(&mut self, value: i32) -> Result<(), String> {
        if value < 0 {
            Err(format!("invalid input, value can not be lower that 0"))
        } else {
            self.input = value;
            Ok(())
        }
    }

    fn get_input(&self) -> i32 {
        self.input
    }

    fn append_request(&mut self, value: &str) {
        self.requests.get_or_insert(vec![]).push(String::from(value));
    }

    fn get_requests(&mut self) -> Option<Vec<String>> {
        self.requests.take()
    }

    fn append_response(&mut self, value: String) {
        self.responses.get_or_insert(vec![]).push(value);
    }

    fn get_responses(&mut self) -> Option<Vec<String>> {
        self.responses.take()
    }

    fn get_control_value(&self) -> i32 {
        self.input + self.output as i32
    }

    fn append_byte_request(&mut self, value: Vec<u8>) {
        self.byte_responses = Some(value);
    }

    fn get_byte_responses(&mut self) -> Option<Vec<u8>> {
        self.byte_responses.take()
    }
}

#[no_mangle]
pub extern fn context_create() -> *mut Context {
    let context = Context::new();

    debug!("context_create {:?}", context);

    Box::into_raw(Box::new(context))
}

#[no_mangle]
pub extern fn context_close(ctx_ptr: *mut Context) {
    if ctx_ptr.is_null() { return }
    let ctx = unsafe { Box::from_raw(ctx_ptr); };
    debug!("context_close {:?}", ctx);
}

#[no_mangle]
pub extern "C" fn context_set_input(ctx_ptr: *mut Context, value: i32) -> bool {
    let ctx = Context::from_ptr(ctx_ptr);
    debug!("setting input {:?} before {:?}",value,  ctx);
    match ctx.set_input(value) {
        Ok(_) => {
            debug!("setting input after {:?}", ctx);
            true
        },
        Err(msg) => {
            debug!(msg);
            false
        },
    }
}

#[no_mangle]
pub extern "C" fn context_add_request(ctx_ptr: *mut Context, value: *const c_char) -> bool {
    let c_str = unsafe {
        assert!(!value.is_null());
        CStr::from_ptr(value)
    };

    let value = c_str.to_str().unwrap();
    let ctx = Context::from_ptr(ctx_ptr);
    debug!("receive input {:?}: {}", ctx.get_control_value(), value);
    ctx.append_request(value);
    true
}

#[no_mangle]
pub extern "C" fn context_execute(ctx_ptr: *mut Context) -> bool {
    let ctx = Context::from_ptr(ctx_ptr);
    debug!("context_execute {:?}", ctx.get_control_value());
    match ctx.get_requests() {
        Some(requests) => {
            for (i, request) in requests.into_iter().enumerate() {
                debug!("processing request {}", request);
                let response = format!("response of {}", request);
                ctx.append_response(response)
            }
        },
        None => {}
    }
    true
}

#[no_mangle]
pub extern "C" fn context_get_responses(ctx_ptr: *mut Context) -> *mut c_char {
    let ctx = Context::from_ptr(ctx_ptr);

    match ctx.get_responses() {
        Some(responses) => {
            let buffer = responses.join("\n");
            let c_str_song = CString::new(buffer).unwrap();
            c_str_song.into_raw()
        },
        _ => {
            let c_str_song = CString::new("").unwrap();
            c_str_song.into_raw()
        }
    }
}


#[no_mangle]
pub extern "C" fn context_set_string(ctx_ptr: *mut Context, value: *const c_char) -> bool {
    let c_str = unsafe {
        assert!(!value.is_null());
        CStr::from_ptr(value)
    };

    let value = c_str.to_str().unwrap();
    let ctx = Context::from_ptr(ctx_ptr);
    debug!("context_set_string {:?}: {}", ctx.get_control_value(), value);
    ctx.string = Some(value.to_string());
    true
}


#[no_mangle]
pub extern "C" fn context_get_string(ctx_ptr: *mut Context) -> *mut c_char {
    let ctx = Context::from_ptr(ctx_ptr);

    let value = ctx.string.take();

    debug!("context_get_string {:?}: {:?}", ctx.get_control_value(), value);

    match value {
        Some(string) => {
            let c_str_song = CString::new(string).unwrap();
            c_str_song.into_raw()
        },
        _ => {
            let c_str_song = CString::new("").unwrap();
            c_str_song.into_raw()
        }
    }
}

#[no_mangle]
pub extern "C" fn context_set_struct(ctx_ptr: *mut Context, value: V2) -> bool {
    let ctx = Context::from_ptr(ctx_ptr);
    debug!("context_set_struct {:?}: {:?}", ctx.get_control_value(), value);
    ctx.v2 = Some(value);
    true
}

#[no_mangle]
pub extern "C" fn context_get_struct(ctx_ptr: *mut Context) -> V2 {
    let ctx = Context::from_ptr(ctx_ptr);
    let value = ctx.v2.take().unwrap_or(V2 { x: 0, y: 0 });
    debug!("context_get_struct {:?}: {:?}", ctx.get_control_value(), value);
    value
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) -> bool {
    assert!(!ptr.is_null());
    unsafe {
        CString::from_raw(ptr);
    };
    true
}

#[no_mangle]
pub extern "C" fn context_add_byte_request(ctx_ptr: *mut Context, buffer: *mut u8, length: i32) -> bool {
    debug!("context_add_byte_request - receiving bytes of {:?}", length);

    assert!(!buffer.is_null(), "buffer pointer could not be null");

    let ref_data = unsafe { std::slice::from_raw_parts(buffer, length as usize) };
    let own_data = ref_data.to_vec();

    let ctx = Context::from_ptr(ctx_ptr);
    ctx.append_byte_request(own_data);

    true
}

#[no_mangle]
pub extern "C" fn context_get_byte_responses(ctx_ptr: *mut Context, callback: extern "stdcall" fn (*mut u8, i32)) -> bool {
    let ctx = Context::from_ptr(ctx_ptr);
    match ctx.get_byte_responses() {
        Some(mut bytes) => {
            callback(bytes.as_mut_ptr(), bytes.len() as i32);
            true
        },
        None => false,
    }
}

#[no_mangle]
pub extern "C" fn context_get_input(ptr: *mut Context) -> i32 {
    let ctx = Context::from_ptr(ptr);
    debug!("getting input as {:?}", ctx);
    ctx.get_input()
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct V2 {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
struct Buffer {
    len: i32,
    data: *mut V2,
}

#[repr(C)]
pub struct Component {
    label: *mut c_char
}

#[repr(C)]
pub struct Entity {
    id: u32,
    pos: V2,
    kind: u32,
    components: *mut Component,
    components_length: u32,
}

#[repr(C)]
pub struct OutputMessages {
    new_entities: *mut Entity,
    new_entities_length: u32,
    removed_entities: *mut u32,
    removed_entities_length: u32,
}

#[repr(C)]
pub struct FFIArray<T> {
    ptr: *const T,
    len: u32
}

#[no_mangle]
pub extern fn add_numbers(number1: i32, number2: i32) -> i32 {
    number1 + number2
}

#[no_mangle]
pub extern "C" fn context_get_new_entities(ctx_ptr: *mut Context, callback: extern "stdcall" fn (FFIArray<Entity>)) -> bool {
    let mut new_entities = vec![
        Entity {
            id: 0,
            pos: V2 {
                x: 0,
                y: 0
            },
            kind: 1,
            components: vec![
                Component {
                    label: CString::new("engine").unwrap().into_raw()
                },
                Component {
                    label: CString::new("weapon").unwrap().into_raw()
                },
            ].as_mut_ptr(),
            components_length: 2,
        },
        Entity {
            id: 2,
            pos: V2 {
                x: 1,
                y: 2
            },
            kind: 3,
            components: vec![
                Component {
                    label: CString::new("sensors").unwrap().into_raw()
                }
            ].as_mut_ptr(),
            components_length: 1,
        },
    ];

    let len = new_entities.len() as u32;

    callback(
        FFIArray {
            ptr: new_entities.as_mut_ptr(),
            len: len
        }
    );

    true
}

#[no_mangle]
pub extern "C" fn context_get_removed_entities(ctx_ptr: *mut Context, callback: extern "stdcall" fn (FFIArray<u32>)) -> bool {
    callback(
        FFIArray {
            ptr: vec![0, 3, 4, 5].as_mut_ptr(),
            len: 4
        }
    );
    true
}

#[no_mangle]
pub extern "C" fn context_get_output_messages(ctx_ptr: *mut Context, callback: extern "stdcall" fn (*mut OutputMessages)) -> bool {
    let ctx = Context::from_ptr(ctx_ptr);

    let mut new_entities = vec![
        Entity {
            id: 0,
            pos: V2 {
                x: 0,
                y: 0
            },
            kind: 0,
            components: vec![
                    Component {
                        label: CString::new("engine").unwrap().into_raw()
                    },
                    Component {
                        label: CString::new("weapon").unwrap().into_raw()
                    },
            ].as_mut_ptr(),
            components_length: 2,
        },
        Entity {
            id: 1,
            pos: V2 {
                x: 1,
                y: 2
            },
            kind: 1,
            components: vec![
                    Component {
                        label: CString::new("sensors").unwrap().into_raw()
                    }
            ].as_mut_ptr(),
            components_length: 1,
        },
    ];

    let mut output = OutputMessages {
        new_entities: new_entities.as_mut_ptr(),
        new_entities_length: 0,
        removed_entities: vec![3].as_mut_ptr(),
        removed_entities_length: 0
    };

    // convert to a pointer and send to callback
    let ptr = Box::into_raw(Box::new(output));
    callback(ptr);

    // restore box to be free
//    let _ = unsafe { Box::from_raw(ptr) };

    true
}


#[no_mangle]
pub extern "C" fn get_simple_struct() -> V2 {
    V2 {
        x: 1,
        y: 2
    }
}

//#[no_mangle]
//extern "C" fn get_vectors(ptr: *mut *const V2) -> bool {
////    let list: [V2; 2] = [V2 { x: 0, y: 1 }, V2 { x: 1, y: 0 }];
////
////    unsafe {
////        *ptr = list.as_ptr();
////    }
////
////    true
//    false
//}

#[no_mangle]
extern "C" fn generate_data() -> Buffer {
    let mut buf = vec![V2 { x: 1, y: 0 }, V2 { x: 2, y: 0}].into_boxed_slice();
    let data = buf.as_mut_ptr();
    let len = buf.len() as i32;
    std::mem::forget(buf);
    Buffer { len, data }
}

#[no_mangle]
extern "C" fn free_buf(buf: Buffer) {
    let s = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len as usize) };
    let s = s.as_mut_ptr();
    unsafe {
        Box::from_raw(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0, 0);
    }
}
