// automatically generated by the FlatBuffers compiler, do not modify

package org.apache.arrow.flatbuf

@Suppress("unused")
@ExperimentalUnsignedTypes
class MetadataVersion private constructor() {
    companion object {
        /**
         * 0.1.0 (October 2016).
         */
        const val V1: Short = 0
        /**
         * 0.2.0 (February 2017). Non-backwards compatible with V1.
         */
        const val V2: Short = 1
        /**
         * 0.3.0 -> 0.7.1 (May - December 2017). Non-backwards compatible with V2.
         */
        const val V3: Short = 2
        /**
         * >= 0.8.0 (December 2017). Non-backwards compatible with V3.
         */
        const val V4: Short = 3
        /**
         * >= 1.0.0 (July 2020. Backwards compatible with V4 (V5 readers can read V4
         * metadata and IPC messages). Implementations are recommended to provide a
         * V4 compatibility mode with V5 format changes disabled.
         *
         * Incompatible changes between V4 and V5:
         * - Union buffer layout has changed. In V5, Unions don't have a validity
         *   bitmap buffer.
         */
        const val V5: Short = 4
        val names : Array<String> = arrayOf("V1", "V2", "V3", "V4", "V5")
        fun name(e: Int) : String = names[e]
    }
}