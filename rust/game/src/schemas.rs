mod requests_generated;
mod responses_generated;

pub use requests_generated::ffi_requests;
pub use responses_generated::ffi_responses;

use crate::GameEvent;
use flatbuffers::FlatBufferBuilder;

pub type RawMsg = [u8];
pub type RawMsgBuffer = Vec<u8>;

#[derive(Debug)]
pub enum SerializationError {
    Unknown(String),
}

pub fn serialize_game_events(
    game_responses: Vec<GameEvent>,
) -> Result<RawMsgBuffer, SerializationError> {
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

    // let mut simple = vec![];
    let mut create_objects = vec![];
    let mut move_objects = vec![];

    for responses in game_responses {
        match responses {
            GameEvent::CreateObj { id, x, y } => {
                create_objects.push(ffi_responses::CreatePackage::new(
                    id,
                    ffi_responses::PrefabKind::Player,
                    x,
                    y,
                ));
            }
            GameEvent::MoveObj { obj_id, x, y } => {
                move_objects.push(ffi_responses::PosPackage::new(obj_id, x, y));
            }
        }
    }

    let args = ffi_responses::ResponsesArgs {
        // simple: create_vector!(simple),
        simple: None,
        create_object: create_vector!(create_objects),
        move_obj: create_vector!(move_objects),
    };

    let out = ffi_responses::Responses::create(&mut fb, &args);
    fb.finish_minimal(out);

    // TODO remove this copy
    Ok(fb.finished_data().to_vec())
}
