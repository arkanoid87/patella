// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum DecimalOffset {}
#[derive(Copy, Clone, PartialEq)]

/// Exact decimal value represented as an integer value in two's
/// complement. Currently only 128-bit (16-byte) and 256-bit (32-byte) integers
/// are used. The representation uses the endianness indicated
/// in the Schema.
pub struct Decimal<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Decimal<'a> {
  type Inner = Decimal<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> Decimal<'a> {
  pub const VT_PRECISION: flatbuffers::VOffsetT = 4;
  pub const VT_SCALE: flatbuffers::VOffsetT = 6;
  pub const VT_BITWIDTH: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Decimal { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args DecimalArgs
  ) -> flatbuffers::WIPOffset<Decimal<'bldr>> {
    let mut builder = DecimalBuilder::new(_fbb);
    builder.add_bitWidth(args.bitWidth);
    builder.add_scale(args.scale);
    builder.add_precision(args.precision);
    builder.finish()
  }


  /// Total number of decimal digits
  #[inline]
  pub fn precision(&self) -> i32 {
    self._tab.get::<i32>(Decimal::VT_PRECISION, Some(0)).unwrap()
  }
  /// Number of digits after the decimal point "."
  #[inline]
  pub fn scale(&self) -> i32 {
    self._tab.get::<i32>(Decimal::VT_SCALE, Some(0)).unwrap()
  }
  /// Number of bits per value. The only accepted widths are 128 and 256.
  /// We use bitWidth for consistency with Int::bitWidth.
  #[inline]
  pub fn bitWidth(&self) -> i32 {
    self._tab.get::<i32>(Decimal::VT_BITWIDTH, Some(128)).unwrap()
  }
}

impl flatbuffers::Verifiable for Decimal<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i32>("precision", Self::VT_PRECISION, false)?
     .visit_field::<i32>("scale", Self::VT_SCALE, false)?
     .visit_field::<i32>("bitWidth", Self::VT_BITWIDTH, false)?
     .finish();
    Ok(())
  }
}
pub struct DecimalArgs {
    pub precision: i32,
    pub scale: i32,
    pub bitWidth: i32,
}
impl<'a> Default for DecimalArgs {
  #[inline]
  fn default() -> Self {
    DecimalArgs {
      precision: 0,
      scale: 0,
      bitWidth: 128,
    }
  }
}
pub struct DecimalBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> DecimalBuilder<'a, 'b> {
  #[inline]
  pub fn add_precision(&mut self, precision: i32) {
    self.fbb_.push_slot::<i32>(Decimal::VT_PRECISION, precision, 0);
  }
  #[inline]
  pub fn add_scale(&mut self, scale: i32) {
    self.fbb_.push_slot::<i32>(Decimal::VT_SCALE, scale, 0);
  }
  #[inline]
  pub fn add_bitWidth(&mut self, bitWidth: i32) {
    self.fbb_.push_slot::<i32>(Decimal::VT_BITWIDTH, bitWidth, 128);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> DecimalBuilder<'a, 'b> {
    let start = _fbb.start_table();
    DecimalBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Decimal<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Decimal<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Decimal");
      ds.field("precision", &self.precision());
      ds.field("scale", &self.scale());
      ds.field("bitWidth", &self.bitWidth());
      ds.finish()
  }
}
