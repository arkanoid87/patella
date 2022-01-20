// automatically generated by the FlatBuffers compiler, do not modify

package org.apache.arrow.flatbuf;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class FixedSizeBinary extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_2_0_0(); }
  public static FixedSizeBinary getRootAsFixedSizeBinary(ByteBuffer _bb) { return getRootAsFixedSizeBinary(_bb, new FixedSizeBinary()); }
  public static FixedSizeBinary getRootAsFixedSizeBinary(ByteBuffer _bb, FixedSizeBinary obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public FixedSizeBinary __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  /**
   * Number of bytes per value
   */
  public int byteWidth() { int o = __offset(4); return o != 0 ? bb.getInt(o + bb_pos) : 0; }

  public static int createFixedSizeBinary(FlatBufferBuilder builder,
      int byteWidth) {
    builder.startTable(1);
    FixedSizeBinary.addByteWidth(builder, byteWidth);
    return FixedSizeBinary.endFixedSizeBinary(builder);
  }

  public static void startFixedSizeBinary(FlatBufferBuilder builder) { builder.startTable(1); }
  public static void addByteWidth(FlatBufferBuilder builder, int byteWidth) { builder.addInt(0, byteWidth, 0); }
  public static int endFixedSizeBinary(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public FixedSizeBinary get(int j) { return get(new FixedSizeBinary(), j); }
    public FixedSizeBinary get(FixedSizeBinary obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
}
