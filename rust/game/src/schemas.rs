mod packages_generated;
mod requests_generated;
mod responses_generated;

pub use requests_generated::ffi_requests;
pub use responses_generated::ffi_responses;

use crate::{Error, GameEvent, Request, Result};
use flatbuffers::FlatBufferBuilder;

pub type RawMsg = [u8];
pub type RawMsgBuffer = Vec<u8>;
pub type PackageKind = u16;

pub fn parse_game_requests(kind: PackageKind, requests: &RawMsg) -> Result<Vec<Request>> {
    if kind != packages_generated::ffi_packages::PackageKind::Request as u16 {
        eprintln!("receive unknown kind {:?}", kind);
        return Err(Error::Unknown(format!("Unknown kind {}", kind)));
    }

    let root = flatbuffers::get_root::<ffi_requests::Requests>(requests);

    let total_requests = root.total_messages() as usize;
    let mut index: Vec<Option<Request>> = Vec::with_capacity(total_requests);
    for i in 0..total_requests {
        index.push(None);
    }

    for package in root.empty_packages().unwrap_or_default().iter() {
        match package.kind() {
            ffi_requests::RequestKind::StartGame => {
                index[package.ordering() as usize] = Some(Request::StartGame)
            }
            ffi_requests::RequestKind::GameStatus => {
                index[package.ordering() as usize] = Some(Request::GameStatus)
            }
            ffi_requests::RequestKind::GetAll => {
                index[package.ordering() as usize] = Some(Request::GetAll)
            }
            other => return Err(Error::Unknown(format!("Invalid kind {:?}", other))),
        }
    }

    let result: Vec<_> = index.into_iter().flatten().collect();

    if result.len() != total_requests {
        Err(format!("invalid result {:?}", result).into())
    } else {
        Ok(result)
    }
}

pub fn serialize_game_events(
    game_responses: Vec<GameEvent>,
) -> Result<(PackageKind, RawMsgBuffer)> {
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
    let mut empty_packages = vec![];
    let mut create_packages = vec![];
    let mut pos_packages = vec![];
    // let mut string_packages = vec![];

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
            GameEvent::GameStarted => empty_packages.push(ffi_responses::EmptyPackage::new(
                ffi_responses::ResponseKind::GameStarted,
                ordering,
            )),
            GameEvent::GameStatusRunning => empty_packages.push(ffi_responses::EmptyPackage::new(
                ffi_responses::ResponseKind::GameStatusRunning,
                ordering,
            )),
            GameEvent::GameStatusIdle => empty_packages.push(ffi_responses::EmptyPackage::new(
                ffi_responses::ResponseKind::GameStatusIdle,
                ordering,
            )),
            GameEvent::FullStateResponse => empty_packages.push(ffi_responses::EmptyPackage::new(
                ffi_responses::ResponseKind::FullStateResponse,
                ordering,
            )),
        }
    }

    if total != total_game_responses as u32 {
        return Err(format!(
            "invalid response count {:?}, expected {:?}",
            total, total_game_responses
        )
        .into());
    }

    let args = ffi_responses::ResponsesArgs {
        total_messages: total,
        empty_packages: create_vector!(empty_packages),
        create_packages: create_vector!(create_packages),
        pos_packages: create_vector!(pos_packages),
        string_packages: None,
    };

    let out = ffi_responses::Responses::create(&mut fb, &args);
    fb.finish_minimal(out);

    // TODO remove this copy
    Ok((
        packages_generated::ffi_packages::PackageKind::Response as u16,
        fb.finished_data().to_vec(),
    ))
}
