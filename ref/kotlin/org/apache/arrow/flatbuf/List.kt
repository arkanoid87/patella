// automatically generated by the FlatBuffers compiler, do not modify

package org.apache.arrow.flatbuf

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

@Suppress("unused")
@ExperimentalUnsignedTypes
class List : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : List {
        __init(_i, _bb)
        return this
    }
    companion object {
        fun validateVersion() = Constants.FLATBUFFERS_2_0_0()
        fun getRootAsList(_bb: ByteBuffer): List = getRootAsList(_bb, List())
        fun getRootAsList(_bb: ByteBuffer, obj: List): List {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        fun startList(builder: FlatBufferBuilder) = builder.startTable(0)
        fun endList(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}