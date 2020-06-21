extern crate ffi_server;
extern crate flatbuffers;

use ffi_server::schemas::responses::*;
use flatbuffers::FlatBufferBuilder;

#[test]
fn test_flatbuffer_non_root_element() {
    let bytes: [u8; 2] = [1, 0];
    let kind = flatbuffers::follow_cast_ref::<ResponseKind>(&bytes, 0);
    assert_eq!(*kind, ResponseKind::CreateObj);
}

#[test]
fn test_flatbuffer_schema_v2_serialization() {
    let bytes = {
        let mut bd = FlatBufferBuilder::new();

        let v = bd.create_vector(&[PosPackage::new(1, 0.2, 3.0)]);

        let package = Responses::create(
            &mut bd,
            &ResponsesArgs {
                simple: None,
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
        let message = flatbuffers::get_root::<Responses>(&bytes);
        assert_eq!(message.move_obj().unwrap().len(), 1);
        assert_eq!(message.move_obj().unwrap()[0].id(), 1);
        assert_eq!(message.move_obj().unwrap()[0].x(), 0.2);
        assert_eq!(message.move_obj().unwrap()[0].y(), 3.0);
    }
}
