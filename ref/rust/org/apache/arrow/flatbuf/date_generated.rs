// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum DateOffset {}
#[derive(Copy, Clone, PartialEq)]

/// Date is either a 32-bit or 64-bit signed integer type representing an
/// elapsed time since UNIX epoch (1970-01-01), stored in either of two units:
///
/// * Milliseconds (64 bits) indicating UNIX time elapsed since the epoch (no
///   leap seconds), where the values are evenly divisible by 86400000
/// * Days (32 bits) since the UNIX epoch
pub struct Date<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Date<'a> {
  type Inner = Date<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> Date<'a> {
  pub const VT_UNIT: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Date { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args DateArgs
  ) -> flatbuffers::WIPOffset<Date<'bldr>> {
    let mut builder = DateBuilder::new(_fbb);
    builder.add_unit(args.unit);
    builder.finish()
  }


  #[inline]
  pub fn unit(&self) -> DateUnit {
    self._tab.get::<DateUnit>(Date::VT_UNIT, Some(DateUnit::MILLISECOND)).unwrap()
  }
}

impl flatbuffers::Verifiable for Date<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<DateUnit>("unit", Self::VT_UNIT, false)?
     .finish();
    Ok(())
  }
}
pub struct DateArgs {
    pub unit: DateUnit,
}
impl<'a> Default for DateArgs {
  #[inline]
  fn default() -> Self {
    DateArgs {
      unit: DateUnit::MILLISECOND,
    }
  }
}
pub struct DateBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> DateBuilder<'a, 'b> {
  #[inline]
  pub fn add_unit(&mut self, unit: DateUnit) {
    self.fbb_.push_slot::<DateUnit>(Date::VT_UNIT, unit, DateUnit::MILLISECOND);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> DateBuilder<'a, 'b> {
    let start = _fbb.start_table();
    DateBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Date<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Date<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Date");
      ds.field("unit", &self.unit());
      ds.finish()
  }
}
