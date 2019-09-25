extern crate flatbuffers;

pub mod schema_generated;
pub mod schema_2_generated;
pub mod monster_schema_generated;

use schema_2_generated as bench_fbs;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use schema_generated::users::{User, UserArgs, finish_user_buffer, get_root_as_user};

pub fn make_user(bldr: &mut FlatBufferBuilder, dest: &mut Vec<u8>, name: &str, id: u64) {
    // Reset the `bytes` Vec to a clean state.
    dest.clear();

    // Reset the `FlatBufferBuilder` to a clean state.
    bldr.reset();

    // Create a temporary `UserArgs` object to build a `User` object.
    // (Note how we call `bldr.create_string` to create the UTF-8 string
    // ergonomically.)
    let args = UserArgs{
        name: Some(bldr.create_string(name)),
        id: id,
    };

    // Call the `User::create` function with the `FlatBufferBuilder` and our
    // UserArgs object, to serialize the data to the FlatBuffer. The returned
    // value is an offset used to track the location of this serializaed data.
    let user_offset = User::create(bldr, &args);

    // Finish the write operation by calling the generated function
    // `finish_user_buffer` with the `user_offset` created by `User::create`.
    finish_user_buffer(bldr, user_offset);

    // Copy the serialized FlatBuffers data to our own byte buffer.
    let finished_data = bldr.finished_data();
    dest.extend_from_slice(finished_data);
}

// src/main.rs part 3 of 4: read_user function
pub fn read_user(buf: &[u8]) -> (&str, u64) {
    let u = get_root_as_user(buf);
    let name = u.name().unwrap();
    let id = u.id();
    (name, id)
}

pub fn main() {

    let mut bldr = FlatBufferBuilder::new();
    let mut bytes: Vec<u8> = Vec::new();

    // Write the provided `name` and `id` into the `bytes` Vec using the
    // FlatBufferBuilder `bldr`:
    make_user(&mut bldr, &mut bytes, "Arthur Dent", 42);

    // Now, `bytes` contains the serialized representation of our User object.

    // To read the serialized data, call our `read_user` function to decode
    // the `user` and `id`:
    let (name, id) = read_user(&bytes[..]);

    // Show the decoded information:
    println!("{} has id {}. The encoded data is {} bytes long.", name, id, bytes.len());
}

#[test]
fn it_deserializes() {
    const ID: u64 = 12;
    let mut builder = FlatBufferBuilder::new();
    // bench::Basic values that will be serialized
    let basic_args = bench_fbs::bench::BasicArgs { id: ID, .. Default::default() };
    // Serialize bench::Basic struct
    let basic: WIPOffset<_> = bench_fbs::bench::Basic::create(&mut builder, &basic_args);
    // Must "finish" the builder before calling `finished_data()`
    builder.finish_minimal(basic);
    // Deserialize the bench::Basic
    let parsed = flatbuffers::get_root::<bench_fbs::bench::Basic>(builder.finished_data());

    println!("receive {:?}", parsed);

    assert_eq!(parsed.id(), ID);
    assert!(false)
}

#[test]
fn it_deserializes_2() {
    const ID: u64 = 12;
    const NAME: &str = "name";
    const REFERENCE: &str = "reference";
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    {
        let args = bench_fbs::bench::BasicArgs{id: ID};
        let basic = Some(bench_fbs::bench::Basic::create(&mut builder, &args));
        let name = Some(builder.create_string(NAME));
        let reference = Some(builder.create_string(REFERENCE));
        let args = bench_fbs::bench::ComplexArgs{ basic, name, reference };
        let complex = bench_fbs::bench::Complex::create(&mut builder, &args);
        builder.finish_minimal(complex);
    }
    let parsed = flatbuffers::get_root::<bench_fbs::bench::Complex>(builder.finished_data());

    println!("receive {:?}", parsed);

    assert_eq!(parsed.basic().id(), ID);
    assert_eq!(parsed.name(), NAME);
    assert_eq!(parsed.reference(), REFERENCE);
    assert!(false)
}
