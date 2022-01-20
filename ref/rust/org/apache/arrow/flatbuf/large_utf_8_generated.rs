// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum LargeUtf8Offset {}
#[derive(Copy, Clone, PartialEq)]

/// Same as Utf8, but with 64-bit offsets, allowing to represent
/// extremely large data values.
pub struct LargeUtf8<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for LargeUtf8<'a> {
  type Inner = LargeUtf8<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> LargeUtf8<'a> {

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    LargeUtf8 { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    _args: &'args LargeUtf8Args
  ) -> flatbuffers::WIPOffset<LargeUtf8<'bldr>> {
    let mut builder = LargeUtf8Builder::new(_fbb);
    builder.finish()
  }

}

impl flatbuffers::Verifiable for LargeUtf8<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .finish();
    Ok(())
  }
}
pub struct LargeUtf8Args {
}
impl<'a> Default for LargeUtf8Args {
  #[inline]
  fn default() -> Self {
    LargeUtf8Args {
    }
  }
}
pub struct LargeUtf8Builder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> LargeUtf8Builder<'a, 'b> {
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> LargeUtf8Builder<'a, 'b> {
    let start = _fbb.start_table();
    LargeUtf8Builder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<LargeUtf8<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for LargeUtf8<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("LargeUtf8");
      ds.finish()
  }
}