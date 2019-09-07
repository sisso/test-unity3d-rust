#[macro_use]
mod debug;

use std::ffi::CStr;
use std::mem::{drop, transmute};
use std::net::UdpSocket;
use std::os::raw::c_char;
use std::ptr;
use std::result::Result;
use std::io::Write;

#[derive(Debug)]
pub struct Context {
    input: i32,
    output: u32,
}

impl<'a> Context {
    fn new() -> Self {
        Context {
            input: 0,
            output: 0,
        }
    }

    fn to_ptr(self) -> *mut Context {
        unsafe { transmute(Box::new(self)) }
    }

    fn from_ptr(ptr: *mut Context) -> &'a mut Context {
        unsafe { &mut *ptr }
    }

    fn close(ptr: *mut *mut Context) {
        let b: Box<Context> = unsafe { transmute(ptr) };
        debug!("close unboxed content {:?}", b);
        drop(b);
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
}

#[no_mangle]
pub extern "C" fn context_create(ptr: *mut *const Context) -> bool {
    let ctx = Context::new();

    debug!("context_create before pointer {:?}", ctx);

    unsafe {
        *ptr = ctx.to_ptr();
    }

    // if failed_to_create {
    // *ptr = ptr::null; false
    // }

    true
}

#[no_mangle]
pub extern "C" fn context_close(ptr: *mut *mut Context) -> bool {
    debug!("receive context_close");

    if !ptr.is_null() && unsafe {!(*ptr).is_null() } {
        debug!("safe to close, do it");

        Context::close(ptr);

        debug!("clean up pointer");

        unsafe {
            *ptr = ptr::null_mut();
        }

        debug!("done");
    }

    true
}

#[no_mangle]
pub extern "C" fn context_set_input(ptr: *mut Context, value: i32) -> bool {
    if ptr.is_null() {
        false
    } else {
        let ctx = Context::from_ptr(ptr);
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
}

#[no_mangle]
pub extern "C" fn context_get_input(ptr: *mut Context) -> i32 {
    if ptr.is_null() {
        0
    } else {
        let ctx = Context::from_ptr(ptr);
        debug!("getting input as {:?}", ctx);
        ctx.get_input()
    }
}

#[repr(C)]
pub struct V2 {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
struct Buffer {
    len: i32,
    data: *mut V2,
}

#[no_mangle]
pub extern fn add_numbers(number1: i32, number2: i32) -> i32 {
    println!("Hello from rust!");
    number1 + number2
}

#[no_mangle]
pub extern "C" fn get_simple_struct() -> V2 {
    V2 {
        x: 1,
        y: 2
    }
}

#[no_mangle]
extern "C" fn get_vectors(ptr: *mut *const V2) -> bool {
//    let list: [V2; 2] = [V2 { x: 0, y: 1 }, V2 { x: 1, y: 0 }];
//
//    unsafe {
//        *ptr = list.as_ptr();
//    }
//
//    true
    false
}

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
        assert_eq!(add_numbers(2, 2), 4);
    }
}
