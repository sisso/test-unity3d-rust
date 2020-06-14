/*
    Implementing the server by FFI
*/

use crate::server::{RawMsg, Result, Server, RawMsgBuffer};

#[derive(Debug)]
pub struct FfiContext {}

impl<'a> FfiContext {
    pub fn new() -> Self {
        FfiContext {}
    }

    pub fn from_ptr(ptr: *mut FfiContext) -> &'a mut FfiContext {
        assert!(!ptr.is_null());

        unsafe { &mut *ptr }
    }
}

impl Server for FfiContext {
    fn push(&mut self, user_id: u16, msg: &RawMsg) -> Result<()> {
        unimplemented!()
    }

    // TODO: receive a closure?
    fn take(&mut self, user_id: u16) -> Result<Vec<RawMsgBuffer>> {
        unimplemented!()
    }
}
