extern crate ffi_server;
extern crate flatbuffers;

use ffi_server::schemas::packages_3_generated::*;
use flatbuffers::FlatBufferBuilder;

#[test]
fn test_flatbuffer_non_root_element() {
    let bytes: [u8; 2] = [1, 0];
    let kind = flatbuffers::follow_cast_ref::<MessageKind>(&bytes, 0);
    assert_eq!(*kind, MessageKind::CreateObj);
}

#[test]
fn test_flatbuffer_schema_v2_serialization() {
    let bytes = {
        let mut bd = FlatBufferBuilder::new();

        let v = bd.create_vector(&[PosPackage::new(1, 0.2, 3.0)]);

        let package = Output::create(
            &mut bd,
            &OutputArgs {
                start_game: None,
                create_object: None,
                move_obj: Some(v),
            },
        );

        bd.finish_minimal(package);
        bd.finished_data().to_vec()
    };

    println!("{:?}", bytes);
    assert_eq!(bytes.len(), 40);

    {
        let message = flatbuffers::get_root::<Output>(&bytes);
        assert_eq!(message.move_obj().unwrap().len(), 1);
        assert_eq!(message.move_obj().unwrap()[0].id(), 1);
        assert_eq!(message.move_obj().unwrap()[0].x(), 0.2);
        assert_eq!(message.move_obj().unwrap()[0].y(), 3.0);
    }
}
