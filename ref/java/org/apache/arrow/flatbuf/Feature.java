// automatically generated by the FlatBuffers compiler, do not modify

package org.apache.arrow.flatbuf;

/**
 * Represents Arrow Features that might not have full support
 * within implementations. This is intended to be used in
 * two scenarios:
 *  1.  A mechanism for readers of Arrow Streams
 *      and files to understand that the stream or file makes
 *      use of a feature that isn't supported or unknown to
 *      the implementation (and therefore can meet the Arrow
 *      forward compatibility guarantees).
 *  2.  A means of negotiating between a client and server
 *      what features a stream is allowed to use. The enums
 *      values here are intented to represent higher level
 *      features, additional details maybe negotiated
 *      with key-value pairs specific to the protocol.
 *
 * Enums added to this list should be assigned power-of-two values
 * to facilitate exchanging and comparing bitmaps for supported
 * features.
 */
@SuppressWarnings("unused")
public final class Feature {
  private Feature() { }
  /**
   * Needed to make flatbuffers happy.
   */
  public static final long UNUSED = 0;
  /**
   * The stream makes use of multiple full dictionaries with the
   * same ID and assumes clients implement dictionary replacement
   * correctly.
   */
  public static final long DICTIONARY_REPLACEMENT = 1;
  /**
   * The stream makes use of compressed bodies as described
   * in Message.fbs.
   */
  public static final long COMPRESSED_BODY = 2;
}

