// automatically generated by the FlatBuffers compiler, do not modify

package org.apache.arrow.flatbuf

@Suppress("unused")
@ExperimentalUnsignedTypes
class UnionMode private constructor() {
    companion object {
        const val Sparse: Short = 0
        const val Dense: Short = 1
        val names : Array<String> = arrayOf("Sparse", "Dense")
        fun name(e: Int) : String = names[e]
    }
}
