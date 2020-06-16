use crate::schema_generated::messages;
use flatbuffers::FlatBufferBuilder;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

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
    pub number: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct FFIPerson {
    pub id: u32,
    pub name: *mut c_char,
    pub number: u32,
}

#[derive(Debug)]
pub struct FfiContext {
    pub control: i32,
    pub string: Option<String>,
    pub v2: Option<V2>,
    pub array: Option<Vec<u8>>,
    pub v2_array: Option<Vec<V2>>,
    pub people: Vec<Person>,
    pub vectors: Vec<(i32, i32)>,
}

impl<'a> FfiContext {
    pub fn new() -> Self {
        FfiContext {
            control: 1,
            string: None,
            v2: None,
            array: None,
            v2_array: None,
            people: vec![],
            vectors: vec![],
        }
    }

    pub fn get_control_value(&self) -> i32 {
        self.control
    }
}
