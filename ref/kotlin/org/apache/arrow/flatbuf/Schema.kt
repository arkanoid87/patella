// automatically generated by the FlatBuffers compiler, do not modify

package org.apache.arrow.flatbuf

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

/**
 * ----------------------------------------------------------------------
 * A Schema describes the columns in a row batch
 */
@Suppress("unused")
@ExperimentalUnsignedTypes
class Schema : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : Schema {
        __init(_i, _bb)
        return this
    }
    /**
     * endianness of the buffer
     * it is Little Endian by default
     * if endianness doesn't match the underlying system then the vectors need to be converted
     */
    val endianness : Short
        get() {
            val o = __offset(4)
            return if(o != 0) bb.getShort(o + bb_pos) else 0
        }
    fun fields(j: Int) : org.apache.arrow.flatbuf.Field? = fields(org.apache.arrow.flatbuf.Field(), j)
    fun fields(obj: org.apache.arrow.flatbuf.Field, j: Int) : org.apache.arrow.flatbuf.Field? {
        val o = __offset(6)
        return if (o != 0) {
            obj.__assign(__indirect(__vector(o) + j * 4), bb)
        } else {
            null
        }
    }
    val fieldsLength : Int
        get() {
            val o = __offset(6); return if (o != 0) __vector_len(o) else 0
        }
    fun customMetadata(j: Int) : org.apache.arrow.flatbuf.KeyValue? = customMetadata(org.apache.arrow.flatbuf.KeyValue(), j)
    fun customMetadata(obj: org.apache.arrow.flatbuf.KeyValue, j: Int) : org.apache.arrow.flatbuf.KeyValue? {
        val o = __offset(8)
        return if (o != 0) {
            obj.__assign(__indirect(__vector(o) + j * 4), bb)
        } else {
            null
        }
    }
    val customMetadataLength : Int
        get() {
            val o = __offset(8); return if (o != 0) __vector_len(o) else 0
        }
    /**
     * Features used in the stream/file.
     */
    fun features(j: Int) : Long {
        val o = __offset(10)
        return if (o != 0) {
            bb.getLong(__vector(o) + j * 8)
        } else {
            0
        }
    }
    val featuresLength : Int
        get() {
            val o = __offset(10); return if (o != 0) __vector_len(o) else 0
        }
    val featuresAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(10, 8)
    fun featuresInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 10, 8)
    companion object {
        fun validateVersion() = Constants.FLATBUFFERS_2_0_0()
        fun getRootAsSchema(_bb: ByteBuffer): Schema = getRootAsSchema(_bb, Schema())
        fun getRootAsSchema(_bb: ByteBuffer, obj: Schema): Schema {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        fun createSchema(builder: FlatBufferBuilder, endianness: Short, fieldsOffset: Int, customMetadataOffset: Int, featuresOffset: Int) : Int {
            builder.startTable(4)
            addFeatures(builder, featuresOffset)
            addCustomMetadata(builder, customMetadataOffset)
            addFields(builder, fieldsOffset)
            addEndianness(builder, endianness)
            return endSchema(builder)
        }
        fun startSchema(builder: FlatBufferBuilder) = builder.startTable(4)
        fun addEndianness(builder: FlatBufferBuilder, endianness: Short) = builder.addShort(0, endianness, 0)
        fun addFields(builder: FlatBufferBuilder, fields: Int) = builder.addOffset(1, fields, 0)
        fun createFieldsVector(builder: FlatBufferBuilder, data: IntArray) : Int {
            builder.startVector(4, data.size, 4)
            for (i in data.size - 1 downTo 0) {
                builder.addOffset(data[i])
            }
            return builder.endVector()
        }
        fun startFieldsVector(builder: FlatBufferBuilder, numElems: Int) = builder.startVector(4, numElems, 4)
        fun addCustomMetadata(builder: FlatBufferBuilder, customMetadata: Int) = builder.addOffset(2, customMetadata, 0)
        fun createCustomMetadataVector(builder: FlatBufferBuilder, data: IntArray) : Int {
            builder.startVector(4, data.size, 4)
            for (i in data.size - 1 downTo 0) {
                builder.addOffset(data[i])
            }
            return builder.endVector()
        }
        fun startCustomMetadataVector(builder: FlatBufferBuilder, numElems: Int) = builder.startVector(4, numElems, 4)
        fun addFeatures(builder: FlatBufferBuilder, features: Int) = builder.addOffset(3, features, 0)
        fun createFeaturesVector(builder: FlatBufferBuilder, data: LongArray) : Int {
            builder.startVector(8, data.size, 8)
            for (i in data.size - 1 downTo 0) {
                builder.addLong(data[i])
            }
            return builder.endVector()
        }
        fun startFeaturesVector(builder: FlatBufferBuilder, numElems: Int) = builder.startVector(8, numElems, 8)
        fun endSchema(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
        fun finishSchemaBuffer(builder: FlatBufferBuilder, offset: Int) = builder.finish(offset)
        fun finishSizePrefixedSchemaBuffer(builder: FlatBufferBuilder, offset: Int) = builder.finishSizePrefixed(offset)
    }
}
