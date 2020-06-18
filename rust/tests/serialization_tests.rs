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
fn test_flatbuffer_schema_v2_serialization() {
    let bytes = {
        let mut bd = FlatBufferBuilder::new();

        let data = PosPackage::create(
            &mut bd,
            &PosPackageArgs {
                id: 1,
                x: 0.2,
                y: 3.0,
            },
        );

        let package = Package::create(
            &mut bd,
            &PackageArgs {
                kind: MessageKind::MoveObj,
                data_type: DataPack::PosPackage,
                data: Some(data.as_union_value()),
            },
        );

        bd.finish_minimal(package);
        bd.finished_data().to_vec()
    };

    println!("{:?}", bytes);
    assert_eq!(56, bytes.len());

    {
        let message = flatbuffers::get_root::<Package>(&bytes);
        assert_eq!(message.kind(), MessageKind::MoveObj);
        assert_eq!(message.data_type(), DataPack::PosPackage);

        let data = message.data_as_pos_package().unwrap();
        assert_eq!(data.id(), 1);
        assert_eq!(data.x(), 0.2);
        assert_eq!(data.y(), 3.0);
    }
}
