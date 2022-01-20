// automatically generated by the FlatBuffers compiler, do not modify

package org.apache.arrow.flatbuf;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class Duration extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_2_0_0(); }
  public static Duration getRootAsDuration(ByteBuffer _bb) { return getRootAsDuration(_bb, new Duration()); }
  public static Duration getRootAsDuration(ByteBuffer _bb, Duration obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public Duration __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public short unit() { int o = __offset(4); return o != 0 ? bb.getShort(o + bb_pos) : 1; }

  public static int createDuration(FlatBufferBuilder builder,
      short unit) {
    builder.startTable(1);
    Duration.addUnit(builder, unit);
    return Duration.endDuration(builder);
  }

  public static void startDuration(FlatBufferBuilder builder) { builder.startTable(1); }
  public static void addUnit(FlatBufferBuilder builder, short unit) { builder.addShort(0, unit, 1); }
  public static int endDuration(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public Duration get(int j) { return get(new Duration(), j); }
    public Duration get(Duration obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
}

