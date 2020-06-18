extern crate ffi_server;
extern crate flatbuffers;

use ffi_server::schemas::packages_2_generated::*;
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

        let package = PosPackage::create(
            &mut bd,
            &PosPackageArgs {
                id: 1,
                x: 0.2,
                y: 3.0,
            },
        );

        bd.finish_minimal(package);
        bd.finished_data().to_vec()
    };

    println!("{:?}", bytes);
    assert_eq!(32, bytes.len());

    {
        let message = flatbuffers::get_root::<PosPackage>(&bytes);
        assert_eq!(message.id(), 1);
        assert_eq!(message.x(), 0.2);
        assert_eq!(message.y(), 3.0);
    }
}
