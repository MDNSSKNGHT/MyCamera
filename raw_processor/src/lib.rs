use std::sync::Arc;

use android_logger::Config;
use jni::{
    JNIEnv,
    objects::{JByteBuffer, JClass},
    sys::{jint, jlong},
};
use log::{LevelFilter, info};
use vulkano::{
    VulkanLibrary,
    device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
};

struct Context {
    device: Arc<Device>,
    queue: Arc<Queue>,
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_mdnssknght_mycamera_processing_NativeRawProcessor_00024Companion_nativeInit(
    mut _env: JNIEnv,
    _class: JClass,
) -> jlong {
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("RustNative"),
    );

    info!("Hello, from Rust!");

    let library = VulkanLibrary::new().expect("Failed to find local Vulkan library");
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    )
    .unwrap();

    let physical_device = instance
        .enumerate_physical_devices()
        .expect("Failed to enumerate physical devices")
        .next()
        .expect("Failed to find physical device");

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_, queue_family_properties)| {
            queue_family_properties
                .queue_flags
                .contains(QueueFlags::COMPUTE)
        })
        .expect("Failed to find a graphical queue family") as u32;

    info!("Queue family with compute {:?}", queue_family_index);

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("Failed to create device");

    let queue = queues.next().unwrap();

    info!("Initialized Vulkan device and queue");

    let context = Arc::new(Context { device, queue });

    Arc::into_raw(context) as jlong
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_mdnssknght_mycamera_processing_NativeRawProcessor_00024Companion_nativeFini(
    mut _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    unsafe { Arc::from_raw(handle as *const Context) };
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_mdnssknght_mycamera_processing_NativeRawProcessor_00024Companion_nativeProcess(
    mut _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    width: jint,
    height: jint,
    _data: JByteBuffer,
) {
    let context = unsafe { Arc::from_raw(handle as *const Context) };

    info!("context: {:?}, {:?}", context.device, context.queue);
    info!("width: {}, height: {}", width, height);

    /* Avoid the inner value from dropping */
    let _ = Arc::into_raw(context);
}
