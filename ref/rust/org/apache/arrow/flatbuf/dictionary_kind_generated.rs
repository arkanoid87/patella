// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_DICTIONARY_KIND: i16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_DICTIONARY_KIND: i16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_DICTIONARY_KIND: [DictionaryKind; 1] = [
  DictionaryKind::DenseArray,
];

/// ----------------------------------------------------------------------
/// Dictionary encoding metadata
/// Maintained for forwards compatibility, in the future
/// Dictionaries might be explicit maps between integers and values
/// allowing for non-contiguous index values
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DictionaryKind(pub i16);
#[allow(non_upper_case_globals)]
impl DictionaryKind {
  pub const DenseArray: Self = Self(0);

  pub const ENUM_MIN: i16 = 0;
  pub const ENUM_MAX: i16 = 0;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::DenseArray,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::DenseArray => Some("DenseArray"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for DictionaryKind {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for DictionaryKind {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<i16>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for DictionaryKind {
    type Output = DictionaryKind;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<i16>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for DictionaryKind {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = i16::to_le(self.0);
    Self(b)
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(self) -> Self {
    let b = i16::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for DictionaryKind {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i16::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for DictionaryKind {}
