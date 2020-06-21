mod ffi_utils;

use crate::client::Client;
use crate::game::GameEvent::CreateObj;
use crate::game::{Game, GameEvent, Result, UserId};
use ffi_utils::*;
use flatbuffers::FlatBufferBuilder;
use game::schemas::{ffi_requests, ffi_responses, RawMsg, RawMsgBuffer};

#[derive(Debug)]
enum RunMode {
    Embedded { game: Game },
    Server { client: Client },
}

#[derive(Debug)]
pub struct FfiContext {
    mode: RunMode,
}

impl<'a> FfiContext {
    pub fn new(address: Option<&str>) -> Self {
        let mode = match address {
            Some(address) => unimplemented!(),
            None => RunMode::Embedded { game: Game::new() },
        };

        FfiContext { mode }
    }

    fn push(&mut self, bytes: &RawMsg) -> Result<()> {
        unimplemented!()
    }

    // TODO: receive a closure?
    fn take(&mut self) -> Result<RawMsgBuffer> {
        match &mut self.mode {
            RunMode::Embedded { game } => {
                let game_responses = game.take();
                game::schemas::serialize_game_events(game_responses)
            }

            RunMode::Server { client } => client.take_responses(),

            _ => unimplemented!(),
        }
    }
}

#[no_mangle]
pub extern "C" fn server_ffi_context_create_embedded() -> Box<FfiContext> {
    let context = FfiContext::new(None);
    debug!("context_create {:?}", context);
    Box::new(context)
}

#[no_mangle]
pub extern "C" fn server_ffi_context_create_and_connect() -> Box<FfiContext> {
    let address = "localhost:28483";
    let context = FfiContext::new(Some(address));
    debug!("context_create {:?} to {:?}", context, address);
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
        Ok(buffer) => {
            callback(buffer.as_ptr(), buffer.len() as u32);
            true
        }

        Err(err) => {
            debug!("server_ffi_take fail: {:?}", err);
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
