// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum Struct_Offset {}
#[derive(Copy, Clone, PartialEq)]

/// A Struct_ in the flatbuffer metadata is the same as an Arrow Struct
/// (according to the physical memory layout). We used Struct_ here as
/// Struct is a reserved word in Flatbuffers
pub struct Struct_<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Struct_<'a> {
  type Inner = Struct_<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> Struct_<'a> {

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Struct_ { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    _args: &'args Struct_Args
  ) -> flatbuffers::WIPOffset<Struct_<'bldr>> {
    let mut builder = Struct_Builder::new(_fbb);
    builder.finish()
  }

}

impl flatbuffers::Verifiable for Struct_<'_> {
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
pub struct Struct_Args {
}
impl<'a> Default for Struct_Args {
  #[inline]
  fn default() -> Self {
    Struct_Args {
    }
  }
}
pub struct Struct_Builder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> Struct_Builder<'a, 'b> {
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> Struct_Builder<'a, 'b> {
    let start = _fbb.start_table();
    Struct_Builder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Struct_<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Struct_<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Struct_");
      ds.finish()
  }
}
