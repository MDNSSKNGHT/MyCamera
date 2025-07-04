package com.mdnssknght.mycamera.processing

import java.nio.ByteBuffer

class NativeRawProcessor {

    companion object {
        init {
            System.loadLibrary("raw_processor")
        }

        external fun nativeInit(): Long

        external fun nativeFini(handle: Long)

        external fun nativeProcess(handle: Long, width: Int, height: Int, data: ByteBuffer)
    }
}