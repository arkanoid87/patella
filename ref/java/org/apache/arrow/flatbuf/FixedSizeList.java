// automatically generated by the FlatBuffers compiler, do not modify

package org.apache.arrow.flatbuf;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class FixedSizeList extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_2_0_0(); }
  public static FixedSizeList getRootAsFixedSizeList(ByteBuffer _bb) { return getRootAsFixedSizeList(_bb, new FixedSizeList()); }
  public static FixedSizeList getRootAsFixedSizeList(ByteBuffer _bb, FixedSizeList obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public FixedSizeList __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  /**
   * Number of list items per value
   */
  public int listSize() { int o = __offset(4); return o != 0 ? bb.getInt(o + bb_pos) : 0; }

  public static int createFixedSizeList(FlatBufferBuilder builder,
      int listSize) {
    builder.startTable(1);
    FixedSizeList.addListSize(builder, listSize);
    return FixedSizeList.endFixedSizeList(builder);
  }

  public static void startFixedSizeList(FlatBufferBuilder builder) { builder.startTable(1); }
  public static void addListSize(FlatBufferBuilder builder, int listSize) { builder.addInt(0, listSize, 0); }
  public static int endFixedSizeList(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public FixedSizeList get(int j) { return get(new FixedSizeList(), j); }
    public FixedSizeList get(FixedSizeList obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
}

