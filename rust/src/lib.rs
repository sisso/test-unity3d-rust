#[repr(C)]
pub struct SharedStuff {
    pub x: i32,
    pub y: i32,
}

#[no_mangle]
pub extern fn add_numbers(number1: i32, number2: i32) -> i32 {
    println!("Hello from rust!");
    number1 + number2
}

#[no_mangle]
pub extern fn get_simple_struct() -> SharedStuff {
    SharedStuff {
        x: 1,
        y: 2
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
