package com.mdnssknght.mycamera.processing

import java.nio.ByteBuffer

class NativeRawProcessor {

    companion object {
        init {
            System.loadLibrary("raw_processor")
            init()
        }

        external fun init()

        external fun process(width: Int, height: Int, data: ByteBuffer)
    }
}