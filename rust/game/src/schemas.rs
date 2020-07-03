mod requests_generated;
mod responses_generated;

pub use requests_generated::ffi_requests;
pub use responses_generated::ffi_responses;

use crate::{GameEvent, Result};
use flatbuffers::FlatBufferBuilder;

pub type RawMsg = [u8];
pub type RawMsgBuffer = Vec<u8>;

pub fn serialize_game_events(game_responses: Vec<GameEvent>) -> Result<RawMsgBuffer> {
    // TODO move buffer to context for reuse
    let mut fb = FlatBufferBuilder::new();

    macro_rules! create_vector {
        ($field:expr) => {
            if $field.is_empty() {
                None
            } else {
                Some(fb.create_vector($field.as_ref()))
            }
        };
    }

    let mut total = 0u32;
    // let mut empty_packages = vec![];
    let mut create_packages = vec![];
    let mut pos_packages = vec![];

    let total_game_responses = game_responses.len();
    for responses in game_responses {
        let ordering = total;
        total += 1;

        match responses {
            GameEvent::CreateObj { id, x, y } => {
                create_packages.push(ffi_responses::CreatePackage::new(
                    ffi_responses::ResponseKind::CreateObj,
                    ordering,
                    id,
                    ffi_responses::PrefabKind::Player,
                    x,
                    y,
                ));
            }
            GameEvent::MoveObj { obj_id, x, y } => {
                pos_packages.push(ffi_responses::PosPackage::new(
                    ffi_responses::ResponseKind::MoveObj,
                    ordering,
                    obj_id,
                    x,
                    y,
                ));
            }
        }
    }

    if total != total_game_responses as u32 {
        panic!("invalid response count");
    }

    let args = ffi_responses::ResponsesArgs {
        total_messages: total,
        empty_packages: None,
        create_packages: create_vector!(create_packages),
        pos_packages: create_vector!(pos_packages),
    };

    let out = ffi_responses::Responses::create(&mut fb, &args);
    fb.finish_minimal(out);

    // TODO remove this copy
    Ok(fb.finished_data().to_vec())
}
