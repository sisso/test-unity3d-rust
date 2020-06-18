extern crate ffi_server;
extern crate flatbuffers;

use ffi_server::schemas::packages_generated::*;
use flatbuffers::FlatBufferBuilder;

#[test]
fn test_flatbuffer_non_root_element() {
    let bytes: [u8; 2] = [1, 0];
    let kind = flatbuffers::follow_cast_ref::<MessageKind>(&bytes, 0);
    assert_eq!(*kind, MessageKind::CreateObj);
}

#[test]
fn test_flatbuffer_schema_serialization() {
    let bytes = {
        let mut bd = FlatBufferBuilder::new();

        let package = Package::create(
            &mut bd,
            &PackageArgs {
                kind: MessageKind::MoveObj,
                id: Some(&IdPackage::new(1)),
                pos: Some(&PosPackage::new(0.2, 3.0)),
                str: None,
                bytes: None,
            },
        );

        bd.finish_minimal(package);
        bd.finished_data().to_vec()
    };

    println!("{:?}", bytes);
    assert_eq!(36, bytes.len());

    {
        let message = flatbuffers::get_root::<Package>(&bytes);
        assert_eq!(message.kind(), MessageKind::MoveObj);
        assert_eq!(message.id().unwrap().id(), 1);
        assert_eq!(message.pos().unwrap().x(), 0.2);
        assert_eq!(message.pos().unwrap().y(), 3.0);
    }
}
