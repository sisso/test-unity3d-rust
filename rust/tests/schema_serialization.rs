// uncomment clib in Cargo.toml to work

// extern crate flatbuffers;
// extern crate rustlib;
//
// use rustlib::schemas::{server_state_generated, server_generated};
//
// use flatbuffers::{FlatBufferBuilder, WIPOffset};
//
// #[test]
// fn test_schema_serialization() {
//    let bytes = {
//        let mut bd = FlatBufferBuilder::new();
//
//        let kind = server_generated::fb_server::Kind::RequestLogin;
//
//        let points = vec![
//            messages::V2::new(23, 54),
//            messages::V2::new(32, 45),
//        ];
//
//        let outputs = bd.create_vector(points.as_ref());
//
//        let messages_args = messages::MessagesArgs {
//            input: None,
//            output: Some(outputs),
//        };
//
//        let messages = messages::Messages::create(&mut bd, &messages_args);
//        bd.finish_minimal(messages);
//        bd.finished_data().to_vec()
//    };
//
//    println!("{:?}", bytes);
//    assert!(false);
//
//    {
//        let messages = messages::get_root_as_messages(&bytes);
//        assert!(messages.input().is_none());
//        assert!(messages.output().is_some());
//        assert_eq!(messages.output().unwrap()[0].x(), 23);
//        assert_eq!(messages.output().unwrap()[0].y(), 54);
//        assert_eq!(messages.output().unwrap()[1].x(), 32);
//        assert_eq!(messages.output().unwrap()[1].y(), 45);
//    }
// }

