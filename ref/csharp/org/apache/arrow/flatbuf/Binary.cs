// <auto-generated>
//  automatically generated by the FlatBuffers compiler, do not modify
// </auto-generated>

namespace org.apache.arrow.flatbuf
{

using global::System;
using global::System.Collections.Generic;
using global::FlatBuffers;

/// Opaque binary data
public struct Binary : IFlatbufferObject
{
  private Table __p;
  public ByteBuffer ByteBuffer { get { return __p.bb; } }
  public static void ValidateVersion() { FlatBufferConstants.FLATBUFFERS_2_0_0(); }
  public static Binary GetRootAsBinary(ByteBuffer _bb) { return GetRootAsBinary(_bb, new Binary()); }
  public static Binary GetRootAsBinary(ByteBuffer _bb, Binary obj) { return (obj.__assign(_bb.GetInt(_bb.Position) + _bb.Position, _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __p = new Table(_i, _bb); }
  public Binary __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }


  public static void StartBinary(FlatBufferBuilder builder) { builder.StartTable(0); }
  public static Offset<org.apache.arrow.flatbuf.Binary> EndBinary(FlatBufferBuilder builder) {
    int o = builder.EndTable();
    return new Offset<org.apache.arrow.flatbuf.Binary>(o);
  }
}


}
