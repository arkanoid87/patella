// automatically generated by the FlatBuffers compiler, do not modify

package org.apache.arrow.flatbuf;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

/**
 * Time is either a 32-bit or 64-bit signed integer type representing an
 * elapsed time since midnight, stored in either of four units: seconds,
 * milliseconds, microseconds or nanoseconds.
 *
 * The integer `bitWidth` depends on the `unit` and must be one of the following:
 * * SECOND and MILLISECOND: 32 bits
 * * MICROSECOND and NANOSECOND: 64 bits
 *
 * The allowed values are between 0 (inclusive) and 86400 (=24*60*60) seconds
 * (exclusive), adjusted for the time unit (for example, up to 86400000
 * exclusive for the MILLISECOND unit).
 * This definition doesn't allow for leap seconds. Time values from
 * measurements with leap seconds will need to be corrected when ingesting
 * into Arrow (for example by replacing the value 86400 with 86399).
 */
@SuppressWarnings("unused")
public final class Time extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_2_0_0(); }
  public static Time getRootAsTime(ByteBuffer _bb) { return getRootAsTime(_bb, new Time()); }
  public static Time getRootAsTime(ByteBuffer _bb, Time obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public Time __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public short unit() { int o = __offset(4); return o != 0 ? bb.getShort(o + bb_pos) : 1; }
  public int bitWidth() { int o = __offset(6); return o != 0 ? bb.getInt(o + bb_pos) : 32; }

  public static int createTime(FlatBufferBuilder builder,
      short unit,
      int bitWidth) {
    builder.startTable(2);
    Time.addBitWidth(builder, bitWidth);
    Time.addUnit(builder, unit);
    return Time.endTime(builder);
  }

  public static void startTime(FlatBufferBuilder builder) { builder.startTable(2); }
  public static void addUnit(FlatBufferBuilder builder, short unit) { builder.addShort(0, unit, 1); }
  public static void addBitWidth(FlatBufferBuilder builder, int bitWidth) { builder.addInt(1, bitWidth, 32); }
  public static int endTime(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public Time get(int j) { return get(new Time(), j); }
    public Time get(Time obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
}

