// <auto-generated>
//  automatically generated by the FlatBuffers compiler, do not modify
// </auto-generated>

namespace org.apache.arrow.flatbuf
{

using global::System;
using global::System.Collections.Generic;
using global::FlatBuffers;

public struct DictionaryEncoding : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_2_0_0(); }
  public static DictionaryEncoding GetRootAsDictionaryEncoding(ByteBuffer _bb) { return GetRootAsDictionaryEncoding(_bb, new DictionaryEncoding()); }
  public static DictionaryEncoding GetRootAsDictionaryEncoding(ByteBuffer _bb, DictionaryEncoding obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public DictionaryEncoding __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  /// The known dictionary id in the application where this data is used. In
  /// the file or streaming formats, the dictionary ids are found in the
  /// DictionaryBatch messages
  public long Id { get { int o = __p.__offset(4); return o != 0 ? __p.bb.GetLong(o + __p.bb_pos) : (long)0; } }
  /// The dictionary indices are constrained to be non-negative integers. If
  /// this field is null, the indices must be signed int32. To maximize
  /// cross-language compatibility and performance, implementations are
  /// recommended to prefer signed integer types over unsigned integer types
  /// and to avoid uint64 indices unless they are required by an application.
  public org.apache.arrow.flatbuf.Int? IndexType { get { int o = __p.__offset(6); return o != 0 ? (org.apache.arrow.flatbuf.Int?)(new org.apache.arrow.flatbuf.Int()).__assign(__p.__indirect(o + __p.bb_pos), __p.bb) : null; } }
  /// By default, dictionaries are not ordered, or the order does not have
  /// semantic meaning. In some statistical, applications, dictionary-encoding
  /// is used to represent ordered categorical data, and we provide a way to
  /// preserve that metadata here
  public bool IsOrdered { get { int o = __p.__offset(8); return o != 0 ? 0!=__p.bb.Get(o + __p.bb_pos) : (bool)false; } }
  public org.apache.arrow.flatbuf.DictionaryKind DictionaryKind { get { int o = __p.__offset(10); return o != 0 ? (org.apache.arrow.flatbuf.DictionaryKind)__p.bb.GetShort(o + __p.bb_pos) : org.apache.arrow.flatbuf.DictionaryKind.DenseArray; } }

  public static Offset<org.apache.arrow.flatbuf.DictionaryEncoding> CreateDictionaryEncoding(FlatBufferBuilder builder,
      long id = 0,
      Offset<org.apache.arrow.flatbuf.Int> indexTypeOffset = default(Offset<org.apache.arrow.flatbuf.Int>),
      bool isOrdered = false,
      org.apache.arrow.flatbuf.DictionaryKind dictionaryKind = org.apache.arrow.flatbuf.DictionaryKind.DenseArray) {
    builder.StartTable(4);
    DictionaryEncoding.AddId(builder, id);
    DictionaryEncoding.AddIndexType(builder, indexTypeOffset);
    DictionaryEncoding.AddDictionaryKind(builder, dictionaryKind);
    DictionaryEncoding.AddIsOrdered(builder, isOrdered);
    return DictionaryEncoding.EndDictionaryEncoding(builder);
  }

  public static void StartDictionaryEncoding(FlatBufferBuilder builder) { builder.StartTable(4); }
  public static void AddId(FlatBufferBuilder builder, long id) { builder.AddLong(0, id, 0); }
  public static void AddIndexType(FlatBufferBuilder builder, Offset<org.apache.arrow.flatbuf.Int> indexTypeOffset) { builder.AddOffset(1, indexTypeOffset.Value, 0); }
  public static void AddIsOrdered(FlatBufferBuilder builder, bool isOrdered) { builder.AddBool(2, isOrdered, false); }
  public static void AddDictionaryKind(FlatBufferBuilder builder, org.apache.arrow.flatbuf.DictionaryKind dictionaryKind) { builder.AddShort(3, (short)dictionaryKind, 0); }
  public static Offset<org.apache.arrow.flatbuf.DictionaryEncoding> EndDictionaryEncoding(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<org.apache.arrow.flatbuf.DictionaryEncoding>(o);
  }
}


}
