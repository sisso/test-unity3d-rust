#![allow(unused)]

use std::sync::Arc;
use std::sync::Mutex;

pub enum TestEnum {
    A,
    B,
}

#[derive(Clone, PartialEq, Eq)]
pub struct TestClass {
    i: i32,
}

impl TestClass {
    pub fn new() -> TestClass {
        TestClass { i: 0 }
    }

    pub fn increment(&mut self) {
        self.i += 1;
    }

    pub fn add(&mut self, i: i32) {
        self.i += i;
    }

    pub fn add_ref(&mut self, i: &i32) {
        self.i += *i;
    }

    pub fn maybe_add(&mut self, i: Option<i32>) -> Option<i32> {
        if let Some(i) = i {
            self.i += i;
            Some(self.i)
        } else {
            None
        }
    }

    pub fn print(&self) {
        println!("TestClass::i: {}", self.i)
    }

    pub fn format(&self) -> String {
        format!("TestClass::i: {}", self.i)
    }

    pub fn get(&self) -> i32 {
        self.i
    }

    pub fn get_ref(&self) -> &i32 {
        &self.i
    }
}

impl std::ops::Drop for TestClass {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

pub struct Template<T> {
    t: T,
}

impl<T> Template<T> {
    fn new(t: T) -> Result<Self, TestError> {
        Ok(Self { t })
    }
}

pub struct TestStaticClass {}

impl TestStaticClass {
    pub fn hello() {
        println!("Hello from Rust");
    }

    pub fn print_number(n: i32) {
        println!("print_number: {}", n);
    }

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn add_ref(a: &i32, b: &i32) -> i32 {
        a + b
    }

    pub fn concat(a: String, b: String) -> String {
        a + &b
    }

    pub fn concat_str(a: &str, b: &str) -> String {
        a.to_owned() + b
    }

    pub fn print_vec_len(mut vec: Vec<i32>) {
        println!("{}", vec.len());
    }

    pub fn print_slice_len(mut vec: &[i32]) {
        println!("{}", vec.len());
    }

    pub fn get_vec() -> Vec<i32> {
        vec![5, 6, 7]
    }

    pub fn return_class_ref(obj: &TestClass) -> &TestClass {
        obj
    }

    pub fn maybe_return_class(str: Option<String>) -> Option<TestClass> {
        str.map(|_| TestClass::new())
    }

    pub fn maybe_return_template() -> Option<Template<i32>> {
        Some(Template { t: 1 })
    }

    pub fn maybe_add_one(i: Option<i32>) -> Option<i32> {
        i.map(|i| i + 1)
    }

    pub fn test_obj_by_value(obj: TestClass) {
        obj.print();
    }

    pub fn try_create_object_template() -> Result<Template<i32>, TestError> {
        Ok(Template { t: 1 })
    }

    pub fn try_empty_result() -> Result<(), TestError> {
        Ok(())
    }

    pub fn try_create_object_ok() -> Result<TestClass, TestError> {
        Ok(TestClass::new())
    }

    pub fn try_create_object_err() -> Result<TestClass, TestError> {
        Err(TestError {})
    }

    pub fn reverse_enum(e: TestEnum) -> TestEnum {
        match e {
            TestEnum::A => TestEnum::B,
            TestEnum::B => TestEnum::A,
        }
    }

    pub fn get_tuple() -> (i32, String) {
        (0, "0".to_owned())
    }
}

#[derive(Debug)]
pub struct TestError {}

impl ::std::error::Error for TestError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        None
    }
}

impl ::std::fmt::Display for TestError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "TestError!")
    }
}

pub struct TestArc {
    i: i32,
}

impl TestArc {
    pub fn new() -> Arc<TestArc> {
        Arc::new(TestArc { i: 0 })
    }

    pub fn to_string(&self) -> String {
        self.i.to_string()
    }

    pub fn to_string_arc(this: Arc<TestArc>) -> String {
        this.i.to_string()
    }

    pub fn to_string_ref_arc(this: &Arc<TestArc>) -> String {
        this.i.to_string()
    }
}

pub struct TestArcMutex {
    i: i32,
}

impl TestArcMutex {
    pub fn new() -> Arc<Mutex<TestArcMutex>> {
        Arc::new(Mutex::new(TestArcMutex { i: 0 }))
    }

    pub fn to_string(&self) -> String {
        self.i.to_string()
    }

    pub fn inc(&mut self) {
        self.i += 1;
    }

    pub fn to_string_arc(this: Arc<Mutex<TestArcMutex>>) -> String {
        this.lock().unwrap().i.to_string()
    }
}

pub struct TestBox {
    i: i32,
}

impl TestBox {
    pub fn new() -> Box<TestBox> {
        Box::new(TestBox { i: 0 })
    }

    pub fn to_string(&self) -> String {
        self.i.to_string()
    }
}

include!(concat!(env!("OUT_DIR"), "/glue.rs"));
