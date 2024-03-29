// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod ffi_responses {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ResponseKind {
  GameStarted = 0,
  GameStatusIdle = 1,
  GameStatusRunning = 2,
  FullStateResponse = 3,
  CreateObj = 4,
  MoveObj = 5,
  InvalidRequest = 6,

}

const ENUM_MIN_RESPONSE_KIND: u16 = 0;
const ENUM_MAX_RESPONSE_KIND: u16 = 6;

impl<'a> flatbuffers::Follow<'a> for ResponseKind {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for ResponseKind {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = u16::to_le(self as u16);
    let p = &n as *const u16 as *const ResponseKind;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = u16::from_le(self as u16);
    let p = &n as *const u16 as *const ResponseKind;
    unsafe { *p }
  }
}

impl flatbuffers::Push for ResponseKind {
    type Output = ResponseKind;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<ResponseKind>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_RESPONSE_KIND:[ResponseKind; 7] = [
  ResponseKind::GameStarted,
  ResponseKind::GameStatusIdle,
  ResponseKind::GameStatusRunning,
  ResponseKind::FullStateResponse,
  ResponseKind::CreateObj,
  ResponseKind::MoveObj,
  ResponseKind::InvalidRequest
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_RESPONSE_KIND:[&'static str; 7] = [
    "GameStarted",
    "GameStatusIdle",
    "GameStatusRunning",
    "FullStateResponse",
    "CreateObj",
    "MoveObj",
    "InvalidRequest"
];

pub fn enum_name_response_kind(e: ResponseKind) -> &'static str {
  let index = e as u16;
  ENUM_NAMES_RESPONSE_KIND[index as usize]
}

#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum PrefabKind {
  Player = 0,
  Monster = 1,

}

const ENUM_MIN_PREFAB_KIND: u16 = 0;
const ENUM_MAX_PREFAB_KIND: u16 = 1;

impl<'a> flatbuffers::Follow<'a> for PrefabKind {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for PrefabKind {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = u16::to_le(self as u16);
    let p = &n as *const u16 as *const PrefabKind;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = u16::from_le(self as u16);
    let p = &n as *const u16 as *const PrefabKind;
    unsafe { *p }
  }
}

impl flatbuffers::Push for PrefabKind {
    type Output = PrefabKind;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<PrefabKind>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_PREFAB_KIND:[PrefabKind; 2] = [
  PrefabKind::Player,
  PrefabKind::Monster
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_PREFAB_KIND:[&'static str; 2] = [
    "Player",
    "Monster"
];

pub fn enum_name_prefab_kind(e: PrefabKind) -> &'static str {
  let index = e as u16;
  ENUM_NAMES_PREFAB_KIND[index as usize]
}

// struct EmptyPackage, aligned to 4
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmptyPackage {
  kind_: ResponseKind,
  padding0__: u16,
  ordering_: u32,
} // pub struct EmptyPackage
impl flatbuffers::SafeSliceAccess for EmptyPackage {}
impl<'a> flatbuffers::Follow<'a> for EmptyPackage {
  type Inner = &'a EmptyPackage;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a EmptyPackage>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a EmptyPackage {
  type Inner = &'a EmptyPackage;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<EmptyPackage>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for EmptyPackage {
    type Output = EmptyPackage;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const EmptyPackage as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b EmptyPackage {
    type Output = EmptyPackage;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const EmptyPackage as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl EmptyPackage {
  pub fn new<'a>(_kind: ResponseKind, _ordering: u32) -> Self {
    EmptyPackage {
      kind_: _kind.to_little_endian(),
      ordering_: _ordering.to_little_endian(),

      padding0__: 0,
    }
  }
  pub fn kind<'a>(&'a self) -> ResponseKind {
    self.kind_.from_little_endian()
  }
  pub fn ordering<'a>(&'a self) -> u32 {
    self.ordering_.from_little_endian()
  }
}

// struct IdPackage, aligned to 4
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdPackage {
  kind_: ResponseKind,
  padding0__: u16,
  ordering_: u32,
  id_: u32,
} // pub struct IdPackage
impl flatbuffers::SafeSliceAccess for IdPackage {}
impl<'a> flatbuffers::Follow<'a> for IdPackage {
  type Inner = &'a IdPackage;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a IdPackage>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a IdPackage {
  type Inner = &'a IdPackage;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<IdPackage>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for IdPackage {
    type Output = IdPackage;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const IdPackage as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b IdPackage {
    type Output = IdPackage;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const IdPackage as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl IdPackage {
  pub fn new<'a>(_kind: ResponseKind, _ordering: u32, _id: u32) -> Self {
    IdPackage {
      kind_: _kind.to_little_endian(),
      ordering_: _ordering.to_little_endian(),
      id_: _id.to_little_endian(),

      padding0__: 0,
    }
  }
  pub fn kind<'a>(&'a self) -> ResponseKind {
    self.kind_.from_little_endian()
  }
  pub fn ordering<'a>(&'a self) -> u32 {
    self.ordering_.from_little_endian()
  }
  pub fn id<'a>(&'a self) -> u32 {
    self.id_.from_little_endian()
  }
}

// struct CreatePackage, aligned to 4
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CreatePackage {
  kind_: ResponseKind,
  padding0__: u16,
  ordering_: u32,
  id_: u32,
  prefab_: PrefabKind,
  padding1__: u16,
  x_: f32,
  y_: f32,
} // pub struct CreatePackage
impl flatbuffers::SafeSliceAccess for CreatePackage {}
impl<'a> flatbuffers::Follow<'a> for CreatePackage {
  type Inner = &'a CreatePackage;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a CreatePackage>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a CreatePackage {
  type Inner = &'a CreatePackage;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<CreatePackage>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for CreatePackage {
    type Output = CreatePackage;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const CreatePackage as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b CreatePackage {
    type Output = CreatePackage;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const CreatePackage as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl CreatePackage {
  pub fn new<'a>(_kind: ResponseKind, _ordering: u32, _id: u32, _prefab: PrefabKind, _x: f32, _y: f32) -> Self {
    CreatePackage {
      kind_: _kind.to_little_endian(),
      ordering_: _ordering.to_little_endian(),
      id_: _id.to_little_endian(),
      prefab_: _prefab.to_little_endian(),
      x_: _x.to_little_endian(),
      y_: _y.to_little_endian(),

      padding0__: 0,
      padding1__: 0,
    }
  }
  pub fn kind<'a>(&'a self) -> ResponseKind {
    self.kind_.from_little_endian()
  }
  pub fn ordering<'a>(&'a self) -> u32 {
    self.ordering_.from_little_endian()
  }
  pub fn id<'a>(&'a self) -> u32 {
    self.id_.from_little_endian()
  }
  pub fn prefab<'a>(&'a self) -> PrefabKind {
    self.prefab_.from_little_endian()
  }
  pub fn x<'a>(&'a self) -> f32 {
    self.x_.from_little_endian()
  }
  pub fn y<'a>(&'a self) -> f32 {
    self.y_.from_little_endian()
  }
}

// struct PosPackage, aligned to 4
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PosPackage {
  kind_: ResponseKind,
  padding0__: u16,
  ordering_: u32,
  id_: u32,
  x_: f32,
  y_: f32,
} // pub struct PosPackage
impl flatbuffers::SafeSliceAccess for PosPackage {}
impl<'a> flatbuffers::Follow<'a> for PosPackage {
  type Inner = &'a PosPackage;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a PosPackage>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a PosPackage {
  type Inner = &'a PosPackage;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<PosPackage>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for PosPackage {
    type Output = PosPackage;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const PosPackage as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b PosPackage {
    type Output = PosPackage;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const PosPackage as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl PosPackage {
  pub fn new<'a>(_kind: ResponseKind, _ordering: u32, _id: u32, _x: f32, _y: f32) -> Self {
    PosPackage {
      kind_: _kind.to_little_endian(),
      ordering_: _ordering.to_little_endian(),
      id_: _id.to_little_endian(),
      x_: _x.to_little_endian(),
      y_: _y.to_little_endian(),

      padding0__: 0,
    }
  }
  pub fn kind<'a>(&'a self) -> ResponseKind {
    self.kind_.from_little_endian()
  }
  pub fn ordering<'a>(&'a self) -> u32 {
    self.ordering_.from_little_endian()
  }
  pub fn id<'a>(&'a self) -> u32 {
    self.id_.from_little_endian()
  }
  pub fn x<'a>(&'a self) -> f32 {
    self.x_.from_little_endian()
  }
  pub fn y<'a>(&'a self) -> f32 {
    self.y_.from_little_endian()
  }
}

// struct V2Package, aligned to 4
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V2Package {
  kind_: ResponseKind,
  padding0__: u16,
  ordering_: u32,
  x_: f32,
  y_: f32,
} // pub struct V2Package
impl flatbuffers::SafeSliceAccess for V2Package {}
impl<'a> flatbuffers::Follow<'a> for V2Package {
  type Inner = &'a V2Package;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a V2Package>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a V2Package {
  type Inner = &'a V2Package;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<V2Package>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for V2Package {
    type Output = V2Package;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const V2Package as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b V2Package {
    type Output = V2Package;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const V2Package as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl V2Package {
  pub fn new<'a>(_kind: ResponseKind, _ordering: u32, _x: f32, _y: f32) -> Self {
    V2Package {
      kind_: _kind.to_little_endian(),
      ordering_: _ordering.to_little_endian(),
      x_: _x.to_little_endian(),
      y_: _y.to_little_endian(),

      padding0__: 0,
    }
  }
  pub fn kind<'a>(&'a self) -> ResponseKind {
    self.kind_.from_little_endian()
  }
  pub fn ordering<'a>(&'a self) -> u32 {
    self.ordering_.from_little_endian()
  }
  pub fn x<'a>(&'a self) -> f32 {
    self.x_.from_little_endian()
  }
  pub fn y<'a>(&'a self) -> f32 {
    self.y_.from_little_endian()
  }
}

pub enum StringPackageOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct StringPackage<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for StringPackage<'a> {
    type Inner = StringPackage<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> StringPackage<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        StringPackage {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args StringPackageArgs<'args>) -> flatbuffers::WIPOffset<StringPackage<'bldr>> {
      let mut builder = StringPackageBuilder::new(_fbb);
      if let Some(x) = args.buffer { builder.add_buffer(x); }
      builder.add_ordering(args.ordering);
      builder.add_kind(args.kind);
      builder.finish()
    }

    pub const VT_KIND: flatbuffers::VOffsetT = 4;
    pub const VT_ORDERING: flatbuffers::VOffsetT = 6;
    pub const VT_BUFFER: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn kind(&self) -> ResponseKind {
    self._tab.get::<ResponseKind>(StringPackage::VT_KIND, Some(ResponseKind::GameStarted)).unwrap()
  }
  #[inline]
  pub fn ordering(&self) -> u32 {
    self._tab.get::<u32>(StringPackage::VT_ORDERING, Some(0)).unwrap()
  }
  #[inline]
  pub fn buffer(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(StringPackage::VT_BUFFER, None)
  }
}

pub struct StringPackageArgs<'a> {
    pub kind: ResponseKind,
    pub ordering: u32,
    pub buffer: Option<flatbuffers::WIPOffset<&'a  str>>,
}
impl<'a> Default for StringPackageArgs<'a> {
    #[inline]
    fn default() -> Self {
        StringPackageArgs {
            kind: ResponseKind::GameStarted,
            ordering: 0,
            buffer: None,
        }
    }
}
pub struct StringPackageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> StringPackageBuilder<'a, 'b> {
  #[inline]
  pub fn add_kind(&mut self, kind: ResponseKind) {
    self.fbb_.push_slot::<ResponseKind>(StringPackage::VT_KIND, kind, ResponseKind::GameStarted);
  }
  #[inline]
  pub fn add_ordering(&mut self, ordering: u32) {
    self.fbb_.push_slot::<u32>(StringPackage::VT_ORDERING, ordering, 0);
  }
  #[inline]
  pub fn add_buffer(&mut self, buffer: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(StringPackage::VT_BUFFER, buffer);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> StringPackageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    StringPackageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<StringPackage<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum BytesPackageOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct BytesPackage<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for BytesPackage<'a> {
    type Inner = BytesPackage<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> BytesPackage<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        BytesPackage {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args BytesPackageArgs<'args>) -> flatbuffers::WIPOffset<BytesPackage<'bldr>> {
      let mut builder = BytesPackageBuilder::new(_fbb);
      if let Some(x) = args.buffer { builder.add_buffer(x); }
      builder.add_ordering(args.ordering);
      builder.add_kind(args.kind);
      builder.finish()
    }

    pub const VT_KIND: flatbuffers::VOffsetT = 4;
    pub const VT_ORDERING: flatbuffers::VOffsetT = 6;
    pub const VT_BUFFER: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn kind(&self) -> ResponseKind {
    self._tab.get::<ResponseKind>(BytesPackage::VT_KIND, Some(ResponseKind::GameStarted)).unwrap()
  }
  #[inline]
  pub fn ordering(&self) -> u32 {
    self._tab.get::<u32>(BytesPackage::VT_ORDERING, Some(0)).unwrap()
  }
  #[inline]
  pub fn buffer(&self) -> Option<&'a [i8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, i8>>>(BytesPackage::VT_BUFFER, None).map(|v| v.safe_slice())
  }
}

pub struct BytesPackageArgs<'a> {
    pub kind: ResponseKind,
    pub ordering: u32,
    pub buffer: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  i8>>>,
}
impl<'a> Default for BytesPackageArgs<'a> {
    #[inline]
    fn default() -> Self {
        BytesPackageArgs {
            kind: ResponseKind::GameStarted,
            ordering: 0,
            buffer: None,
        }
    }
}
pub struct BytesPackageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> BytesPackageBuilder<'a, 'b> {
  #[inline]
  pub fn add_kind(&mut self, kind: ResponseKind) {
    self.fbb_.push_slot::<ResponseKind>(BytesPackage::VT_KIND, kind, ResponseKind::GameStarted);
  }
  #[inline]
  pub fn add_ordering(&mut self, ordering: u32) {
    self.fbb_.push_slot::<u32>(BytesPackage::VT_ORDERING, ordering, 0);
  }
  #[inline]
  pub fn add_buffer(&mut self, buffer: flatbuffers::WIPOffset<flatbuffers::Vector<'b , i8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(BytesPackage::VT_BUFFER, buffer);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> BytesPackageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    BytesPackageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<BytesPackage<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum ResponsesOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Responses<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Responses<'a> {
    type Inner = Responses<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Responses<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Responses {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ResponsesArgs<'args>) -> flatbuffers::WIPOffset<Responses<'bldr>> {
      let mut builder = ResponsesBuilder::new(_fbb);
      if let Some(x) = args.string_packages { builder.add_string_packages(x); }
      if let Some(x) = args.pos_packages { builder.add_pos_packages(x); }
      if let Some(x) = args.create_packages { builder.add_create_packages(x); }
      if let Some(x) = args.empty_packages { builder.add_empty_packages(x); }
      builder.add_total_messages(args.total_messages);
      builder.finish()
    }

    pub const VT_TOTAL_MESSAGES: flatbuffers::VOffsetT = 4;
    pub const VT_EMPTY_PACKAGES: flatbuffers::VOffsetT = 6;
    pub const VT_CREATE_PACKAGES: flatbuffers::VOffsetT = 8;
    pub const VT_POS_PACKAGES: flatbuffers::VOffsetT = 10;
    pub const VT_STRING_PACKAGES: flatbuffers::VOffsetT = 12;

  #[inline]
  pub fn total_messages(&self) -> u32 {
    self._tab.get::<u32>(Responses::VT_TOTAL_MESSAGES, Some(0)).unwrap()
  }
  #[inline]
  pub fn empty_packages(&self) -> Option<&'a [EmptyPackage]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<EmptyPackage>>>(Responses::VT_EMPTY_PACKAGES, None).map(|v| v.safe_slice() )
  }
  #[inline]
  pub fn create_packages(&self) -> Option<&'a [CreatePackage]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<CreatePackage>>>(Responses::VT_CREATE_PACKAGES, None).map(|v| v.safe_slice() )
  }
  #[inline]
  pub fn pos_packages(&self) -> Option<&'a [PosPackage]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<PosPackage>>>(Responses::VT_POS_PACKAGES, None).map(|v| v.safe_slice() )
  }
  #[inline]
  pub fn string_packages(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<StringPackage<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<StringPackage<'a>>>>>(Responses::VT_STRING_PACKAGES, None)
  }
}

pub struct ResponsesArgs<'a> {
    pub total_messages: u32,
    pub empty_packages: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , EmptyPackage>>>,
    pub create_packages: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , CreatePackage>>>,
    pub pos_packages: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , PosPackage>>>,
    pub string_packages: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<StringPackage<'a >>>>>,
}
impl<'a> Default for ResponsesArgs<'a> {
    #[inline]
    fn default() -> Self {
        ResponsesArgs {
            total_messages: 0,
            empty_packages: None,
            create_packages: None,
            pos_packages: None,
            string_packages: None,
        }
    }
}
pub struct ResponsesBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ResponsesBuilder<'a, 'b> {
  #[inline]
  pub fn add_total_messages(&mut self, total_messages: u32) {
    self.fbb_.push_slot::<u32>(Responses::VT_TOTAL_MESSAGES, total_messages, 0);
  }
  #[inline]
  pub fn add_empty_packages(&mut self, empty_packages: flatbuffers::WIPOffset<flatbuffers::Vector<'b , EmptyPackage>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Responses::VT_EMPTY_PACKAGES, empty_packages);
  }
  #[inline]
  pub fn add_create_packages(&mut self, create_packages: flatbuffers::WIPOffset<flatbuffers::Vector<'b , CreatePackage>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Responses::VT_CREATE_PACKAGES, create_packages);
  }
  #[inline]
  pub fn add_pos_packages(&mut self, pos_packages: flatbuffers::WIPOffset<flatbuffers::Vector<'b , PosPackage>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Responses::VT_POS_PACKAGES, pos_packages);
  }
  #[inline]
  pub fn add_string_packages(&mut self, string_packages: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<StringPackage<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Responses::VT_STRING_PACKAGES, string_packages);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ResponsesBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ResponsesBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Responses<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

}  // pub mod FfiResponses

