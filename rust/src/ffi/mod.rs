mod ffi_utils;

use crate::schemas::server_generated::server::PackageKind;
use crate::server::{Event, Server, UserId};
///
/// Map FFI functions to game logic
///
use ffi_utils::*;

pub type RawMsg = [u8];
pub type RawMsgBuffer = Vec<u8>;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct FfiContext {
    user_id: UserId,
    server: Server,
    pending_messages: Vec<RawMsgBuffer>,
}

impl<'a> FfiContext {
    pub fn new() -> Self {
        let mut server: Server = Default::default();
        let user_id = server.connect();

        let pending_messages = vec![FfiContext::response_login()];

        FfiContext {
            user_id,
            server,
            pending_messages,
        }
    }

    fn response_login() -> Vec<u8> {
        vec![PackageKind::ResponseLogin as u8]
    }

    fn push(&mut self, bytes: &RawMsg) -> Result<()> {
        unimplemented!()
    }

    // TODO: receive a closure?
    fn take(&mut self) -> Result<Vec<RawMsgBuffer>> {
        let mut vec = std::mem::replace(&mut self.pending_messages, Vec::new());
        let server_msg = self.server.take(self.user_id).into_iter().flat_map(|msg| {
            match FfiContext::serialize_event(msg) {
                Ok(bytes) => Some(bytes),
                Err(e) => {
                    // TODO: replace by log
                    eprintln!("fail {:?}", e);
                    None
                }
            }
        });
        vec.extend(server_msg);
        Ok(vec)
    }

    fn serialize_event(event: Event) -> Result<RawMsgBuffer> {
        unimplemented!()
    }

    fn parse_event(bytes: &RawMsg) -> Result<Event> {
        unimplemented!()
    }
}

#[no_mangle]
pub extern "C" fn server_ffi_context_create() -> Box<FfiContext> {
    let context = FfiContext::new();
    debug!("context_create {:?}", context);
    Box::new(context)
}

#[no_mangle]
pub extern "C" fn server_ffi_context_close(ctx: Box<FfiContext>) {
    debug!("context_close {:?}", ctx);
}

pub extern "C" fn server_ffi_push(ctx: &mut FfiContext, buffer: *mut u8, length: u32) -> bool {
    let ref_data = to_slice(buffer, length);
    // debug!("server_ffi_push {:?}: {:?}", ctx.get_control_value(), value);
    match ctx.push(ref_data) {
        Err(err) => {
            debug!("server_ffi_push fail: {:?}", err);
            false
        }
        _ => true,
    }
}

#[no_mangle]
pub extern "C" fn server_ffi_take(
    ctx: &mut FfiContext,
    callback: extern "stdcall" fn(*const u8, u32),
) -> bool {
    // debug!("server_ffi_push {:?}: {:?}", ctx.get_control_value(), value);
    match ctx.take() {
        Ok(messages) => {
            for msg in messages {
                callback(msg.as_ptr(), msg.len() as u32);
            }
            true
        }

        Err(err) => {
            debug!("server_ffi_take fail: {:?}", err);
            false
        }
    }
}
