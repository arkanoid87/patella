// <auto-generated>
//  automatically generated by the FlatBuffers compiler, do not modify
// </auto-generated>

namespace org.apache.arrow.flatbuf
{

using global::System;
using global::System.Collections.Generic;
using global::FlatBuffers;

public struct FloatingPoint : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_2_0_0(); }
  public static FloatingPoint GetRootAsFloatingPoint(ByteBuffer _bb) { return GetRootAsFloatingPoint(_bb, new FloatingPoint()); }
  public static FloatingPoint GetRootAsFloatingPoint(ByteBuffer _bb, FloatingPoint obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public FloatingPoint __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public org.apache.arrow.flatbuf.Precision Precision { get { int o = __p.__offset(4); return o != 0 ? (org.apache.arrow.flatbuf.Precision)__p.bb.GetShort(o + __p.bb_pos) : org.apache.arrow.flatbuf.Precision.HALF; } }

  public static Offset<org.apache.arrow.flatbuf.FloatingPoint> CreateFloatingPoint(FlatBufferBuilder builder,
      org.apache.arrow.flatbuf.Precision precision = org.apache.arrow.flatbuf.Precision.HALF) {
    builder.StartTable(1);
    FloatingPoint.AddPrecision(builder, precision);
    return FloatingPoint.EndFloatingPoint(builder);
  }

  public static void StartFloatingPoint(FlatBufferBuilder builder) { builder.StartTable(1); }
  public static void AddPrecision(FlatBufferBuilder builder, org.apache.arrow.flatbuf.Precision precision) { builder.AddShort(0, (short)precision, 0); }
  public static Offset<org.apache.arrow.flatbuf.FloatingPoint> EndFloatingPoint(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<org.apache.arrow.flatbuf.FloatingPoint>(o);
  }
}


}
