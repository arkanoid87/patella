// automatically generated by the FlatBuffers compiler, do not modify

package org.apache.arrow.flatbuf

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

@Suppress("unused")
@ExperimentalUnsignedTypes
class Bool : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : Bool {
        __init(_i, _bb)
        return this
    }
    companion object {
        fun validateVersion() = Constants.FLATBUFFERS_2_0_0()
        fun getRootAsBool(_bb: ByteBuffer): Bool = getRootAsBool(_bb, Bool())
        fun getRootAsBool(_bb: ByteBuffer, obj: Bool): Bool {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        fun startBool(builder: FlatBufferBuilder) = builder.startTable(0)
        fun endBool(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}
