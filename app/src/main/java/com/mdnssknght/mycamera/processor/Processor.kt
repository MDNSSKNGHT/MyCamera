package com.mdnssknght.mycamera.processor

import java.nio.ByteBuffer

class Processor {

    companion object {
        init {
            System.loadLibrary("processor")
        }

        /** Basic processor for RAW byte data. */
        external fun processRaw(data: ByteBuffer, width: Int, height: Int)
    }
}