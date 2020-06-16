extern crate flatbuffers;
extern crate rustlib;

use rustlib::schema_generated;

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use rustlib::schema_generated::messages::V2;
use schema_generated::messages;

#[test]
fn test_non_root_element() {
    let bytes: [u8; 8] = [1, 0, 0, 0, 3, 0, 0, 0];
    let v2 = flatbuffers::follow_cast_ref::<V2>(&bytes, 0);
    assert_eq!(v2.x(), 1);
    assert_eq!(v2.y(), 3);
}

#[test]
fn test_schema_serialization() {
    let bytes = {
        let mut bd = FlatBufferBuilder::new();

        let points = vec![messages::V2::new(23, 54), messages::V2::new(32, 45)];

        let outputs = bd.create_vector(points.as_ref());

        let messages_args = messages::MessagesArgs {
            input: None,
            output: Some(outputs),
        };

        let messages = messages::Messages::create(&mut bd, &messages_args);
        bd.finish_minimal(messages);
        bd.finished_data().to_vec()
    };

    // println!("{:?}", bytes);
    // assert!(false);

    {
        let messages = messages::get_root_as_messages(&bytes);
        assert!(messages.input().is_none());
        assert!(messages.output().is_some());
        assert_eq!(messages.output().unwrap()[0].x(), 23);
        assert_eq!(messages.output().unwrap()[0].y(), 54);
        assert_eq!(messages.output().unwrap()[1].x(), 32);
        assert_eq!(messages.output().unwrap()[1].y(), 45);
    }
}
