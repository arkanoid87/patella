// automatically generated by the FlatBuffers compiler, do not modify

package org.apache.arrow.flatbuf

/**
 * ----------------------------------------------------------------------
 * Endianness of the platform producing the data
 */
@Suppress("unused")
@ExperimentalUnsignedTypes
class Endianness private constructor() {
    companion object {
        const val Little: Short = 0
        const val Big: Short = 1
        val names : Array<String> = arrayOf("Little", "Big")
        fun name(e: Int) : String = names[e]
    }
}