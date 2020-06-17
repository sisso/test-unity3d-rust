mod ffi_utils;

use crate::game::{Game, Message, UserId};
use crate::schemas::packages_generated::{EmptyPackage, MessageKind, SimplePackage};
///
/// Map FFI functions to game logic
///
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
    user_id: UserId,
    game: Game,
    pending_messages: Vec<RawMsgBuffer>,
}

impl<'a> FfiContext {
    pub fn new() -> Self {
        let mut game: Game = Game::new();
        let user_id = game.connect();

        FfiContext {
            user_id,
            game,
            pending_messages: Default::default(),
        }
    }

    fn push(&mut self, bytes: &RawMsg) -> Result<()> {
        unimplemented!()
    }

    // TODO: receive a closure?
    fn take(&mut self) -> Result<Vec<RawMsgBuffer>> {
        let mut vec = std::mem::replace(&mut self.pending_messages, Vec::new());
        let server_msg = self.game.take(self.user_id).into_iter().flat_map(|msg| {
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

    fn serialize_event(message: Message) -> Result<RawMsgBuffer> {
        // let mut bd = FlatBufferBuilder::new();
        //
        // match message {
        //     Message::StartGame => {
        //         package_args.kind = PackageKind::StartGame;
        //     }
        //     Message::CreateObj { id } => {
        //         package_args.kind = PackageKind::CreateObj;
        //         package_args.id = id;
        //     }
        //     Message::MoveObj { obj_id, x, y } => {
        //         package_args.kind = PackageKind::MoveObj;
        //         package_args.x = x;
        //         package_args.y = y;
        //     }
        //     Message::SetInputAxis { hor, ver } => {
        //         package_args.kind = PackageKind::SetInputAxis;
        //         package_args.x = hor;
        //         package_args.y = ver;
        //     }
        // }
        //
        // let package = Package::create(&mut bd, &package_args);
        // bd.finish_minimal(package);
        //
        // let bytes = bd.finished_data().to_vec();
        // Ok(bytes)
        unimplemented!()
    }

    fn parse_event(bytes: &RawMsg) -> Result<Message> {
        // let kind = parse_kind(bytes);
        unimplemented!()
    }

    fn parse_kind(bytes: &RawMsg) {
        unimplemented!();
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::Message;

    #[test]
    fn test_serialize_and_parse_package() -> Result<()> {
        let message = Message::MoveObj {
            obj_id: 1,
            x: 0.2,
            y: 3.0,
        };

        let bytes = FfiContext::serialize_event(message)?;

        let message_2 = FfiContext::parse_event(bytes.as_slice())?;

        match message_2 {
            Message::MoveObj { obj_id, x, y } => {
                assert_eq!(obj_id, 1);
                assert_eq!(x, 0.2);
                assert_eq!(y, 0.3);
            }
            other => panic!("unexpected {:?}", other),
        }

        Ok(())
    }
}
