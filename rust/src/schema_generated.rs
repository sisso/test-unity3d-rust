// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod messages {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

// struct V2, aligned to 4
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V2 {
  x_: i32,
  y_: i32,
} // pub struct V2
impl flatbuffers::SafeSliceAccess for V2 {}
impl<'a> flatbuffers::Follow<'a> for V2 {
  type Inner = &'a V2;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a V2>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a V2 {
  type Inner = &'a V2;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<V2>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for V2 {
    type Output = V2;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const V2 as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b V2 {
    type Output = V2;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const V2 as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl V2 {
  pub fn new<'a>(_x: i32, _y: i32) -> Self {
    V2 {
      x_: _x.to_little_endian(),
      y_: _y.to_little_endian(),

    }
  }
  pub fn x<'a>(&'a self) -> i32 {
    self.x_.from_little_endian()
  }
  pub fn y<'a>(&'a self) -> i32 {
    self.y_.from_little_endian()
  }
}

pub enum MessagesOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Messages<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Messages<'a> {
    type Inner = Messages<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Messages<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Messages {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MessagesArgs<'args>) -> flatbuffers::WIPOffset<Messages<'bldr>> {
      let mut builder = MessagesBuilder::new(_fbb);
      if let Some(x) = args.output { builder.add_output(x); }
      if let Some(x) = args.input { builder.add_input(x); }
      builder.finish()
    }

    pub const VT_INPUT: flatbuffers::VOffsetT = 4;
    pub const VT_OUTPUT: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn input(&self) -> Option<&'a [V2]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<V2>>>(Messages::VT_INPUT, None).map(|v| v.safe_slice() )
  }
  #[inline]
  pub fn output(&self) -> Option<&'a [V2]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<V2>>>(Messages::VT_OUTPUT, None).map(|v| v.safe_slice() )
  }
}

pub struct MessagesArgs<'a> {
    pub input: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , V2>>>,
    pub output: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , V2>>>,
}
impl<'a> Default for MessagesArgs<'a> {
    #[inline]
    fn default() -> Self {
        MessagesArgs {
            input: None,
            output: None,
        }
    }
}
pub struct MessagesBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessagesBuilder<'a, 'b> {
  #[inline]
  pub fn add_input(&mut self, input: flatbuffers::WIPOffset<flatbuffers::Vector<'b , V2>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Messages::VT_INPUT, input);
  }
  #[inline]
  pub fn add_output(&mut self, output: flatbuffers::WIPOffset<flatbuffers::Vector<'b , V2>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Messages::VT_OUTPUT, output);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessagesBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessagesBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Messages<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

#[inline]
pub fn get_root_as_messages<'a>(buf: &'a [u8]) -> Messages<'a> {
  flatbuffers::get_root::<Messages<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_messages<'a>(buf: &'a [u8]) -> Messages<'a> {
  flatbuffers::get_size_prefixed_root::<Messages<'a>>(buf)
}

#[inline]
pub fn finish_messages_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Messages<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_messages_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Messages<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod messages

