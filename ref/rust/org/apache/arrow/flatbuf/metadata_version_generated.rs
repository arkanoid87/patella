// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_METADATA_VERSION: i16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_METADATA_VERSION: i16 = 4;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_METADATA_VERSION: [MetadataVersion; 5] = [
  MetadataVersion::V1,
  MetadataVersion::V2,
  MetadataVersion::V3,
  MetadataVersion::V4,
  MetadataVersion::V5,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct MetadataVersion(pub i16);
#[allow(non_upper_case_globals)]
impl MetadataVersion {
  /// 0.1.0 (October 2016).
  pub const V1: Self = Self(0);
  /// 0.2.0 (February 2017). Non-backwards compatible with V1.
  pub const V2: Self = Self(1);
  /// 0.3.0 -> 0.7.1 (May - December 2017). Non-backwards compatible with V2.
  pub const V3: Self = Self(2);
  /// >= 0.8.0 (December 2017). Non-backwards compatible with V3.
  pub const V4: Self = Self(3);
  /// >= 1.0.0 (July 2020. Backwards compatible with V4 (V5 readers can read V4
  /// metadata and IPC messages). Implementations are recommended to provide a
  /// V4 compatibility mode with V5 format changes disabled.
  ///
  /// Incompatible changes between V4 and V5:
  /// - Union buffer layout has changed. In V5, Unions don't have a validity
  ///   bitmap buffer.
  pub const V5: Self = Self(4);

  pub const ENUM_MIN: i16 = 0;
  pub const ENUM_MAX: i16 = 4;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::V1,
    Self::V2,
    Self::V3,
    Self::V4,
    Self::V5,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::V1 => Some("V1"),
      Self::V2 => Some("V2"),
      Self::V3 => Some("V3"),
      Self::V4 => Some("V4"),
      Self::V5 => Some("V5"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for MetadataVersion {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for MetadataVersion {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<i16>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for MetadataVersion {
    type Output = MetadataVersion;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<i16>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for MetadataVersion {
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

impl<'a> flatbuffers::Verifiable for MetadataVersion {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i16::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for MetadataVersion {}
