mod ffi_utils;

use crate::game::GameEvent::CreateObj;
use crate::game::{Game, GameEvent, UserId};
use ffi_utils::*;
use flatbuffers::FlatBufferBuilder;
use game::schemas::{ffi_requests, ffi_responses, RawMsg, RawMsgBuffer, SerializationError};

#[derive(Debug)]
pub struct FfiContext {
    // TODO: game should be a parameters, the same for the messages and how to convert MSG to Pacakge
    game: Game,
}

impl<'a> FfiContext {
    pub fn new() -> Self {
        let mut game: Game = Game::new();

        FfiContext { game }
    }

    fn push(&mut self, bytes: &RawMsg) -> Result<(), SerializationError> {
        unimplemented!()
    }

    // TODO: receive a closure?
    fn take(&mut self) -> Result<RawMsgBuffer, SerializationError> {
        let game_responses = self.game.take();
        game::schemas::serialize_game_events(game_responses)
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
