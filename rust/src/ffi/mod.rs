mod ffi_utils;

use crate::game::Responses::CreateObj;
use crate::game::{Game, Responses, UserId};
use crate::schemas::{requests, responses};
use ffi_utils::*;
use flatbuffers::FlatBufferBuilder;

pub type RawMsg = [u8];
pub type RawMsgBuffer = Vec<u8>;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;

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

    fn push(&mut self, bytes: &RawMsg) -> Result<()> {
        unimplemented!()
    }

    // TODO: receive a closure?
    fn take(&mut self) -> Result<RawMsgBuffer> {
        // TODO move buffer to context for reuse
        let mut fb = FlatBufferBuilder::new();

        macro_rules! create_vector {
            ($field:expr) => {
                if $field.is_empty() {
                    None
                } else {
                    let v = std::mem::replace(&mut $field, vec![]);
                    Some(fb.create_vector(v.as_ref()))
                }
            };
        }

        let mut simple = vec![];
        let mut create_objects = vec![];
        let mut move_objects = vec![];

        for responses in self.game.take() {
            match responses {
                Responses::StartGame => simple.push(responses::EmptyPackage::new(
                    responses::ResponseKind::StartGame,
                )),
                Responses::CreateObj { id, x, y } => {
                    create_objects.push(responses::CreatePackage::new(
                        id,
                        responses::PrefabKind::Player,
                        x,
                        y,
                    ));
                }
                Responses::MoveObj { obj_id, x, y } => {
                    move_objects.push(responses::PosPackage::new(obj_id, x, y));
                }
            }
        }

        let args = responses::ResponsesArgs {
            simple: create_vector!(simple),
            create_object: create_vector!(create_objects),
            move_obj: create_vector!(move_objects),
        };

        responses::Responses::create(&mut fb, &args);

        // TODO remove this copy
        Ok(fb.finished_data().to_vec())
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
