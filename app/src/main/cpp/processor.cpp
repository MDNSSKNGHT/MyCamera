//
// Created by mdnssknght on 06/06/2024.
//

#include <jni.h>

#define LOG_TAG "MyProcessor"
#include "logging.h"

extern "C"
JNIEXPORT void JNICALL
Java_com_mdnssknght_mycamera_processor_Processor_00024Companion_processRaw(JNIEnv *env,
                                                                           jobject /* this */,
                                                                           jobject data, jint width,
                                                                           jint height) {
    LOGD("RAW width: %d", width);
    LOGD("RAW height: %d", height);
}