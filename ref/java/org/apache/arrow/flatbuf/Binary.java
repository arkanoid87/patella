// automatically generated by the FlatBuffers compiler, do not modify

package org.apache.arrow.flatbuf;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

/**
 * Opaque binary data
 */
@SuppressWarnings("unused")
public final class Binary extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_2_0_0(); }
  public static Binary getRootAsBinary(ByteBuffer _bb) { return getRootAsBinary(_bb, new Binary()); }
  public static Binary getRootAsBinary(ByteBuffer _bb, Binary obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public Binary __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }


  public static void startBinary(FlatBufferBuilder builder) { builder.startTable(0); }
  public static int endBinary(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public Binary get(int j) { return get(new Binary(), j); }
    public Binary get(Binary obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
}

