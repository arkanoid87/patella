// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum SchemaOffset {}
#[derive(Copy, Clone, PartialEq)]

/// ----------------------------------------------------------------------
/// A Schema describes the columns in a row batch
pub struct Schema<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Schema<'a> {
  type Inner = Schema<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> Schema<'a> {
  pub const VT_ENDIANNESS: flatbuffers::VOffsetT = 4;
  pub const VT_FIELDS: flatbuffers::VOffsetT = 6;
  pub const VT_CUSTOM_METADATA: flatbuffers::VOffsetT = 8;
  pub const VT_FEATURES: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Schema { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args SchemaArgs<'args>
  ) -> flatbuffers::WIPOffset<Schema<'bldr>> {
    let mut builder = SchemaBuilder::new(_fbb);
    if let Some(x) = args.features { builder.add_features(x); }
    if let Some(x) = args.custom_metadata { builder.add_custom_metadata(x); }
    if let Some(x) = args.fields { builder.add_fields(x); }
    builder.add_endianness(args.endianness);
    builder.finish()
  }


  /// endianness of the buffer
  /// it is Little Endian by default
  /// if endianness doesn't match the underlying system then the vectors need to be converted
  #[inline]
  pub fn endianness(&self) -> Endianness {
    self._tab.get::<Endianness>(Schema::VT_ENDIANNESS, Some(Endianness::Little)).unwrap()
  }
  #[inline]
  pub fn fields(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Field<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Field>>>>(Schema::VT_FIELDS, None)
  }
  #[inline]
  pub fn custom_metadata(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue>>>>(Schema::VT_CUSTOM_METADATA, None)
  }
  /// Features used in the stream/file.
  #[inline]
  pub fn features(&self) -> Option<flatbuffers::Vector<'a, Feature>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, Feature>>>(Schema::VT_FEATURES, None)
  }
}

impl flatbuffers::Verifiable for Schema<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<Endianness>("endianness", Self::VT_ENDIANNESS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Field>>>>("fields", Self::VT_FIELDS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<KeyValue>>>>("custom_metadata", Self::VT_CUSTOM_METADATA, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, Feature>>>("features", Self::VT_FEATURES, false)?
     .finish();
    Ok(())
  }
}
pub struct SchemaArgs<'a> {
    pub endianness: Endianness,
    pub fields: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Field<'a>>>>>,
    pub custom_metadata: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>>>,
    pub features: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, Feature>>>,
}
impl<'a> Default for SchemaArgs<'a> {
  #[inline]
  fn default() -> Self {
    SchemaArgs {
      endianness: Endianness::Little,
      fields: None,
      custom_metadata: None,
      features: None,
    }
  }
}
pub struct SchemaBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> SchemaBuilder<'a, 'b> {
  #[inline]
  pub fn add_endianness(&mut self, endianness: Endianness) {
    self.fbb_.push_slot::<Endianness>(Schema::VT_ENDIANNESS, endianness, Endianness::Little);
  }
  #[inline]
  pub fn add_fields(&mut self, fields: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Field<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Schema::VT_FIELDS, fields);
  }
  #[inline]
  pub fn add_custom_metadata(&mut self, custom_metadata: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<KeyValue<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Schema::VT_CUSTOM_METADATA, custom_metadata);
  }
  #[inline]
  pub fn add_features(&mut self, features: flatbuffers::WIPOffset<flatbuffers::Vector<'b , Feature>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Schema::VT_FEATURES, features);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SchemaBuilder<'a, 'b> {
    let start = _fbb.start_table();
    SchemaBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Schema<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Schema<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Schema");
      ds.field("endianness", &self.endianness());
      ds.field("fields", &self.fields());
      ds.field("custom_metadata", &self.custom_metadata());
      ds.field("features", &self.features());
      ds.finish()
  }
}
#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_schema<'a>(buf: &'a [u8]) -> Schema<'a> {
  unsafe { flatbuffers::root_unchecked::<Schema<'a>>(buf) }
}

#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_schema<'a>(buf: &'a [u8]) -> Schema<'a> {
  unsafe { flatbuffers::size_prefixed_root_unchecked::<Schema<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `Schema`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_schema_unchecked`.
pub fn root_as_schema(buf: &[u8]) -> Result<Schema, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Schema>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Schema` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_schema_unchecked`.
pub fn size_prefixed_root_as_schema(buf: &[u8]) -> Result<Schema, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Schema>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Schema` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_schema_unchecked`.
pub fn root_as_schema_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Schema<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Schema<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Schema` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_schema_unchecked`.
pub fn size_prefixed_root_as_schema_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Schema<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Schema<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Schema and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Schema`.
pub unsafe fn root_as_schema_unchecked(buf: &[u8]) -> Schema {
  flatbuffers::root_unchecked::<Schema>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Schema and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Schema`.
pub unsafe fn size_prefixed_root_as_schema_unchecked(buf: &[u8]) -> Schema {
  flatbuffers::size_prefixed_root_unchecked::<Schema>(buf)
}
#[inline]
pub fn finish_schema_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Schema<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_schema_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Schema<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
