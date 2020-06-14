/*
    Implementing the server by FFI
*/

use crate::server::{RawMsg, RawMsgBuffer, Result, Server, ServerConnector};

#[derive(Debug)]
pub struct FfiContext {
    /// embedded server
    server: Server,
}

impl<'a> FfiContext {
    pub fn new() -> Self {
        FfiContext {
            server: Default::default(),
        }
    }

    pub fn from_ptr(ptr: *mut FfiContext) -> &'a mut FfiContext {
        assert!(!ptr.is_null());

        unsafe { &mut *ptr }
    }
}

impl ServerConnector for FfiContext {
    fn push(&mut self, user_id: u16, msg: &RawMsg) -> Result<()> {
        unimplemented!()
    }

    // TODO: receive a closure?
    fn take(&mut self, user_id: u16) -> Result<Vec<RawMsgBuffer>> {
        unimplemented!()
    }
}
