use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub fn to_cstr(value: &str) -> *mut c_char {
    CString::new(value).unwrap().into_raw()
}

pub fn from_cstr(ptr: *const c_char) -> String {
    let c_str = unsafe {
        assert!(!ptr.is_null());
        CStr::from_ptr(ptr)
    };

    c_str.to_str().unwrap().to_string()
}

pub fn to_slice<'a, T>(buffer: *const T, length: u32) -> &'a [T] {
    unsafe { std::slice::from_raw_parts(buffer, length as usize) }
}
