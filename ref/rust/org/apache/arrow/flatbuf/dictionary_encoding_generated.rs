// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum DictionaryEncodingOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct DictionaryEncoding<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for DictionaryEncoding<'a> {
  type Inner = DictionaryEncoding<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> DictionaryEncoding<'a> {
  pub const VT_ID: flatbuffers::VOffsetT = 4;
  pub const VT_INDEXTYPE: flatbuffers::VOffsetT = 6;
  pub const VT_ISORDERED: flatbuffers::VOffsetT = 8;
  pub const VT_DICTIONARYKIND: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    DictionaryEncoding { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args DictionaryEncodingArgs<'args>
  ) -> flatbuffers::WIPOffset<DictionaryEncoding<'bldr>> {
    let mut builder = DictionaryEncodingBuilder::new(_fbb);
    builder.add_id(args.id);
    if let Some(x) = args.indexType { builder.add_indexType(x); }
    builder.add_dictionaryKind(args.dictionaryKind);
    builder.add_isOrdered(args.isOrdered);
    builder.finish()
  }


  /// The known dictionary id in the application where this data is used. In
  /// the file or streaming formats, the dictionary ids are found in the
  /// DictionaryBatch messages
  #[inline]
  pub fn id(&self) -> i64 {
    self._tab.get::<i64>(DictionaryEncoding::VT_ID, Some(0)).unwrap()
  }
  /// The dictionary indices are constrained to be non-negative integers. If
  /// this field is null, the indices must be signed int32. To maximize
  /// cross-language compatibility and performance, implementations are
  /// recommended to prefer signed integer types over unsigned integer types
  /// and to avoid uint64 indices unless they are required by an application.
  #[inline]
  pub fn indexType(&self) -> Option<Int<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<Int>>(DictionaryEncoding::VT_INDEXTYPE, None)
  }
  /// By default, dictionaries are not ordered, or the order does not have
  /// semantic meaning. In some statistical, applications, dictionary-encoding
  /// is used to represent ordered categorical data, and we provide a way to
  /// preserve that metadata here
  #[inline]
  pub fn isOrdered(&self) -> bool {
    self._tab.get::<bool>(DictionaryEncoding::VT_ISORDERED, Some(false)).unwrap()
  }
  #[inline]
  pub fn dictionaryKind(&self) -> DictionaryKind {
    self._tab.get::<DictionaryKind>(DictionaryEncoding::VT_DICTIONARYKIND, Some(DictionaryKind::DenseArray)).unwrap()
  }
}

impl flatbuffers::Verifiable for DictionaryEncoding<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i64>("id", Self::VT_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<Int>>("indexType", Self::VT_INDEXTYPE, false)?
     .visit_field::<bool>("isOrdered", Self::VT_ISORDERED, false)?
     .visit_field::<DictionaryKind>("dictionaryKind", Self::VT_DICTIONARYKIND, false)?
     .finish();
    Ok(())
  }
}
pub struct DictionaryEncodingArgs<'a> {
    pub id: i64,
    pub indexType: Option<flatbuffers::WIPOffset<Int<'a>>>,
    pub isOrdered: bool,
    pub dictionaryKind: DictionaryKind,
}
impl<'a> Default for DictionaryEncodingArgs<'a> {
  #[inline]
  fn default() -> Self {
    DictionaryEncodingArgs {
      id: 0,
      indexType: None,
      isOrdered: false,
      dictionaryKind: DictionaryKind::DenseArray,
    }
  }
}
pub struct DictionaryEncodingBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> DictionaryEncodingBuilder<'a, 'b> {
  #[inline]
  pub fn add_id(&mut self, id: i64) {
    self.fbb_.push_slot::<i64>(DictionaryEncoding::VT_ID, id, 0);
  }
  #[inline]
  pub fn add_indexType(&mut self, indexType: flatbuffers::WIPOffset<Int<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Int>>(DictionaryEncoding::VT_INDEXTYPE, indexType);
  }
  #[inline]
  pub fn add_isOrdered(&mut self, isOrdered: bool) {
    self.fbb_.push_slot::<bool>(DictionaryEncoding::VT_ISORDERED, isOrdered, false);
  }
  #[inline]
  pub fn add_dictionaryKind(&mut self, dictionaryKind: DictionaryKind) {
    self.fbb_.push_slot::<DictionaryKind>(DictionaryEncoding::VT_DICTIONARYKIND, dictionaryKind, DictionaryKind::DenseArray);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> DictionaryEncodingBuilder<'a, 'b> {
    let start = _fbb.start_table();
    DictionaryEncodingBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<DictionaryEncoding<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for DictionaryEncoding<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("DictionaryEncoding");
      ds.field("id", &self.id());
      ds.field("indexType", &self.indexType());
      ds.field("isOrdered", &self.isOrdered());
      ds.field("dictionaryKind", &self.dictionaryKind());
      ds.finish()
  }
}
