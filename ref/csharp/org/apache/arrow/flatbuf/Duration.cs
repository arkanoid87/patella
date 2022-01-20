// <auto-generated>
//  automatically generated by the FlatBuffers compiler, do not modify
// </auto-generated>

namespace org.apache.arrow.flatbuf
{

using global::System;
using global::System.Collections.Generic;
using global::FlatBuffers;

public struct Duration : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_2_0_0(); }
  public static Duration GetRootAsDuration(ByteBuffer _bb) { return GetRootAsDuration(_bb, new Duration()); }
  public static Duration GetRootAsDuration(ByteBuffer _bb, Duration obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public Duration __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public org.apache.arrow.flatbuf.TimeUnit Unit { get { int o = __p.__offset(4); return o != 0 ? (org.apache.arrow.flatbuf.TimeUnit)__p.bb.GetShort(o + __p.bb_pos) : org.apache.arrow.flatbuf.TimeUnit.MILLISECOND; } }

  public static Offset<org.apache.arrow.flatbuf.Duration> CreateDuration(FlatBufferBuilder builder,
      org.apache.arrow.flatbuf.TimeUnit unit = org.apache.arrow.flatbuf.TimeUnit.MILLISECOND) {
    builder.StartTable(1);
    Duration.AddUnit(builder, unit);
    return Duration.EndDuration(builder);
  }

  public static void StartDuration(FlatBufferBuilder builder) { builder.StartTable(1); }
  public static void AddUnit(FlatBufferBuilder builder, org.apache.arrow.flatbuf.TimeUnit unit) { builder.AddShort(0, (short)unit, 1); }
  public static Offset<org.apache.arrow.flatbuf.Duration> EndDuration(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<org.apache.arrow.flatbuf.Duration>(o);
  }
}


}