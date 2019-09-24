use std::ffi::{CString, CStr};
use std::os::raw::c_char;

#[macro_use]
mod debug;

#[derive(Debug)]
pub struct Context {
    control: i32,
    string: Option<String>,
    v2: Option<V2>,
    array: Option<Vec<u8>>,
    v2_array: Option<Vec<V2>>,
    people: Vec<Person>
}

impl<'a> Context {
    fn new() -> Self {
        Context {
            control: 1,
            string: None,
            v2: None,
            array: None,
            v2_array: None,
            people: vec![]
        }
    }

    fn from_ptr(ptr: *mut Context) -> &'a mut Context {
        assert!(!ptr.is_null());

        unsafe {
            &mut *ptr
        }
    }


    fn get_control_value(&self) -> i32 {
        self.control
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
pub extern "C" fn context_set_array(ctx_ptr: *mut Context, buffer: *mut u8, length: u32) -> bool {
    let ctx = Context::from_ptr(ctx_ptr);
    let ref_data = unsafe { std::slice::from_raw_parts(buffer, length as usize) };
    let value = ref_data.to_vec();
    debug!("context_set_array {:?}: {:?}", ctx.get_control_value(), value);
    ctx.array = Some(value);
    true
}

#[no_mangle]
pub extern "C" fn context_get_array(ctx_ptr: *mut Context, callback: extern "stdcall" fn (*mut u8, u32)) -> bool {
    let ctx = Context::from_ptr(ctx_ptr);

    match ctx.array.take() {
        Some(mut value) => {
            debug!("context_get_array {:?}: {:?}", ctx.get_control_value(), value);
            callback(value.as_mut_ptr(), value.len() as u32);
            true
        },
        None => false,
    }
}

#[no_mangle]
pub extern "C" fn context_set_struct_array(ctx_ptr: *mut Context, buffer: *mut V2, length: u32) -> bool {
    let ctx = Context::from_ptr(ctx_ptr);
    let ref_data = to_slice(buffer, length);
    let value = ref_data.to_vec();
    debug!("context_set_struct_array {:?}: {:?}", ctx.get_control_value(), value);
    ctx.v2_array = Some(value);
    true
}

#[no_mangle]
pub extern "C" fn context_get_struct_array(ctx_ptr: *mut Context, callback: extern "stdcall" fn (*mut V2, u32)) -> bool {
    let ctx = Context::from_ptr(ctx_ptr);

    match ctx.v2_array.take() {
        Some(mut value) => {
            debug!("context_get_struct_array {:?}: {:?}", ctx.get_control_value(), value);
            callback(value.as_mut_ptr(), value.len() as u32);
            true
        },
        None => false,
    }
}

#[no_mangle]
pub extern "C" fn context_set_people(ctx_ptr: *mut Context, buffer: *const FFIPerson, length: u32) -> bool {
    let ctx = Context::from_ptr(ctx_ptr);
    let ref_data = to_slice(buffer, length);
    debug!("context_set_persons {:?}: {:?}", ctx.get_control_value(), ref_data);

    let people = ref_data.iter().map(|ffi_person| {
        Person {
            id: ffi_person.id,
            name: from_cstr(ffi_person.name),
            addresses: to_slice(ffi_person.addresses, ffi_person.addresses_length).iter().map(|ffi_address| {
                Address {
                    address: from_cstr(ffi_address.address)
                }
            }).collect()
        }
    }).collect();

    ctx.people = people;
    true
}

#[no_mangle]
pub extern "C" fn context_get_people(ctx_ptr: *mut Context, callback: extern "stdcall" fn (*mut FFIPerson, u32)) -> bool {
    let ctx = Context::from_ptr(ctx_ptr);

    let people = &ctx.people;
    let len= people.len() as u32;

    let mut people_ffi = people.into_iter().map(|person|{
        FFIPerson {
            id: person.id,
            name: to_cstr(person.name.as_str()),
            addresses: person.addresses.iter().map(|address| {
                FFIAddress {
                    address: to_cstr(address.address.as_str())
                }
            }).collect::<Vec<_>>().as_mut_ptr(),
            addresses_length: person.addresses.len() as u32
        }
    }).collect::<Vec<_>>();

    callback(people_ffi.as_mut_ptr(), len);

    true
}

fn to_cstr(value: &str) -> *mut c_char{
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
pub extern "C" fn free_string(ptr: *mut c_char) -> bool {
    assert!(!ptr.is_null());
    unsafe {
        CString::from_raw(ptr);
    };
    true
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct V2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub addresses: Vec<Address>
}

#[derive(Debug)]
pub struct Address {
    pub address: String,
}

#[repr(C)]
#[derive(Debug)]
pub struct FFIPerson {
    pub id: u32,
    pub name: *mut c_char,
    pub addresses: *mut FFIAddress,
    pub addresses_length: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct FFIAddress {
    pub address: *mut c_char,
}

#[no_mangle]
pub extern fn add_numbers(number1: i32, number2: i32) -> i32 {
    number1 + number2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0, 0);
    }
}
