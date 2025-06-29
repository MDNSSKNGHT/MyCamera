use android_logger::Config;
use jni::{
    JNIEnv,
    objects::{JByteBuffer, JClass},
    sys::jint,
};
use log::{LevelFilter, info};

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_mdnssknght_mycamera_processing_NativeRawProcessor_00024Companion_init(
    mut _env: JNIEnv,
    _class: JClass,
) {
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("RustNative"),
    );

    info!("Hello, from Rust!");
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_mdnssknght_mycamera_processing_NativeRawProcessor_00024Companion_process(
    mut _env: JNIEnv,
    _class: JClass,
    width: jint,
    height: jint,
    _data: JByteBuffer,
) {
    info!("width: {}, height: {}", width, height);
}
