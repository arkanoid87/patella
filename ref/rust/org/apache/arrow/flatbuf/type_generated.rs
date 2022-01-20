// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_TYPE: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_TYPE: u8 = 21;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_TYPE: [Type; 22] = [
  Type::NONE,
  Type::Null,
  Type::Int,
  Type::FloatingPoint,
  Type::Binary,
  Type::Utf8,
  Type::Bool,
  Type::Decimal,
  Type::Date,
  Type::Time,
  Type::Timestamp,
  Type::Interval,
  Type::List,
  Type::Struct_,
  Type::Union,
  Type::FixedSizeBinary,
  Type::FixedSizeList,
  Type::Map,
  Type::Duration,
  Type::LargeBinary,
  Type::LargeUtf8,
  Type::LargeList,
];

/// ----------------------------------------------------------------------
/// Top-level Type value, enabling extensible type-specific metadata. We can
/// add new logical types to Type without breaking backwards compatibility
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Type(pub u8);
#[allow(non_upper_case_globals)]
impl Type {
  pub const NONE: Self = Self(0);
  pub const Null: Self = Self(1);
  pub const Int: Self = Self(2);
  pub const FloatingPoint: Self = Self(3);
  pub const Binary: Self = Self(4);
  pub const Utf8: Self = Self(5);
  pub const Bool: Self = Self(6);
  pub const Decimal: Self = Self(7);
  pub const Date: Self = Self(8);
  pub const Time: Self = Self(9);
  pub const Timestamp: Self = Self(10);
  pub const Interval: Self = Self(11);
  pub const List: Self = Self(12);
  pub const Struct_: Self = Self(13);
  pub const Union: Self = Self(14);
  pub const FixedSizeBinary: Self = Self(15);
  pub const FixedSizeList: Self = Self(16);
  pub const Map: Self = Self(17);
  pub const Duration: Self = Self(18);
  pub const LargeBinary: Self = Self(19);
  pub const LargeUtf8: Self = Self(20);
  pub const LargeList: Self = Self(21);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 21;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::Null,
    Self::Int,
    Self::FloatingPoint,
    Self::Binary,
    Self::Utf8,
    Self::Bool,
    Self::Decimal,
    Self::Date,
    Self::Time,
    Self::Timestamp,
    Self::Interval,
    Self::List,
    Self::Struct_,
    Self::Union,
    Self::FixedSizeBinary,
    Self::FixedSizeList,
    Self::Map,
    Self::Duration,
    Self::LargeBinary,
    Self::LargeUtf8,
    Self::LargeList,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::Null => Some("Null"),
      Self::Int => Some("Int"),
      Self::FloatingPoint => Some("FloatingPoint"),
      Self::Binary => Some("Binary"),
      Self::Utf8 => Some("Utf8"),
      Self::Bool => Some("Bool"),
      Self::Decimal => Some("Decimal"),
      Self::Date => Some("Date"),
      Self::Time => Some("Time"),
      Self::Timestamp => Some("Timestamp"),
      Self::Interval => Some("Interval"),
      Self::List => Some("List"),
      Self::Struct_ => Some("Struct_"),
      Self::Union => Some("Union"),
      Self::FixedSizeBinary => Some("FixedSizeBinary"),
      Self::FixedSizeList => Some("FixedSizeList"),
      Self::Map => Some("Map"),
      Self::Duration => Some("Duration"),
      Self::LargeBinary => Some("LargeBinary"),
      Self::LargeUtf8 => Some("LargeUtf8"),
      Self::LargeList => Some("LargeList"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for Type {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for Type {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<u8>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for Type {
    type Output = Type;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<u8>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for Type {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = u8::to_le(self.0);
    Self(b)
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(self) -> Self {
    let b = u8::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for Type {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Type {}
pub struct TypeUnionTableOffset {}

