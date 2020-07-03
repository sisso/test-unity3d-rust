extern crate ffi_domain;
extern crate flatbuffers;
extern crate game;

use ffi_domain::ffi::FfiContext;
use flatbuffers::FlatBufferBuilder;
use game::schemas::ffi_responses::*;
use game::Game;

#[test]
fn test_flatbuffer_non_root_element() {
    let bytes: [u8; 2] = [0, 0];
    let kind = flatbuffers::follow_cast_ref::<ResponseKind>(&bytes, 0);
    assert_eq!(*kind, ResponseKind::GameStarted);
}

#[test]
fn test_flatbuffer_schema_serialization() {
    let bytes = {
        let mut bd = FlatBufferBuilder::new();

        let v = bd.create_vector(&[PosPackage::new(ResponseKind::MoveObj, 0, 1, 0.2, 3.0)]);

        let package = Responses::create(
            &mut bd,
            &ResponsesArgs {
                total_messages: 1,
                empty_packages: None,
                create_packages: None,
                pos_packages: Some(v),
            },
        );

        bd.finish_minimal(package);
        bd.finished_data().to_vec()
    };

    // just to know the size, can be updated if fail
    assert_eq!(bytes.len(), 52);

    {
        let message = flatbuffers::get_root::<Responses>(&bytes);
        assert_eq!(message.total_messages(), 1);
        assert_eq!(message.pos_packages().unwrap().len(), 1);
        assert_eq!(message.pos_packages().unwrap()[0].id(), 1);
        assert_eq!(message.pos_packages().unwrap()[0].x(), 0.2);
        assert_eq!(message.pos_packages().unwrap()[0].y(), 3.0);
    }
}
