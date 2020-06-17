extern crate ffi_server;
extern crate flatbuffers;

use ffi_server::schemas::packages_generated::*;
use flatbuffers::FlatBufferBuilder;

#[test]
fn test_non_root_element() {
    let bytes: [u8; 2] = [1, 0];
    let kind = flatbuffers::follow_cast_ref::<MessageKind>(&bytes, 0);
    assert_eq!(*kind, MessageKind::CreateObj);
}

#[test]
fn test_schema_serialization() {
    let bytes = {
        let mut bd = FlatBufferBuilder::new();

        let package = SimplePackage::create(
            &mut bd,
            &SimplePackageArgs {
                kind: MessageKind::MoveObj,
                id: 1,
                x: 0.2,
                y: 3.0,
            },
        );

        let outer = Package::create(
            &mut bd,
            &PackageArgs {
                package_type: GenericPackage::SimplePackage,
                package: Some(package.as_union_value()),
            },
        );

        bd.finish_minimal(outer);
        bd.finished_data().to_vec()
    };

    println!("{:?}", bytes);

    {
        let message = get_root_as_package(&bytes);
        assert_eq!(GenericPackage::SimplePackage, message.package_type());

        let message = message.package_as_simple_package().unwrap();
        assert_eq!(message.kind(), MessageKind::MoveObj);
        assert_eq!(message.id(), 1);
        assert_eq!(message.x(), 0.2);
        assert_eq!(message.y(), 3.0);
    }
}

#[test]
fn test_schema_serialization_invalid() {
    let bytes = {
        let mut bd = FlatBufferBuilder::new();

        let package = EmptyPackage::create(
            &mut bd,
            &EmptyPackageArgs {
                kind: MessageKind::MoveObj,
            },
        );

        let outer = Package::create(
            &mut bd,
            &PackageArgs {
                package_type: GenericPackage::EmptyPackage,
                package: Some(package.as_union_value()),
            },
        );

        bd.finish_minimal(outer);
        bd.finished_data().to_vec()
    };

    println!("{:?}", bytes);

    {
        let message = get_root_as_package(&bytes);

        let message = message.package_as_empty_package().unwrap();
        assert_eq!(message.kind(), MessageKind::MoveObj);
    }
}

#[test]
fn manually() {}
