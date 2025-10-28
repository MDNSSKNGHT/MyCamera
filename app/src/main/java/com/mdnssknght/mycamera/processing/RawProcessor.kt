package com.mdnssknght.mycamera.processing

import java.nio.ByteBuffer

object RawProcessor {
    private var pointerHandle: Long = 0

    init {
        pointerHandle = NativeRawProcessor.nativeInit()
    }

    fun init() {
        // Because this is an object we want the pointer to the handle to be initialized
        // only once.
    }

    fun fini() {
        NativeRawProcessor.nativeFini(pointerHandle)
    }

    fun process(width: Int, height: Int, data: ByteBuffer, out: ByteArray) {
        NativeRawProcessor.nativeProcess(pointerHandle, width, height, data, out)
    }
}