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
