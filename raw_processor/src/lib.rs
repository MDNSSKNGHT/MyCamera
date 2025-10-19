use std::{panic, slice, sync::Arc};

use android_logger::Config;
use jni::{
    JNIEnv,
    objects::{JByteBuffer, JClass},
    sys::{jint, jlong},
};
use log::{LevelFilter, error, info};
use vulkano::{
    VulkanLibrary,
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferToImageInfo,
        allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
    },
    device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags},
    format::Format,
    image::{Image, ImageCreateInfo, ImageType, ImageUsage},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    sync::{self, GpuFuture},
};

struct Context {
    device: Arc<Device>,
    queue: Arc<Queue>,
    memory_allocator: Arc<StandardMemoryAllocator>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
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

    panic::set_hook(Box::new(move |panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            error!("panic occurred: {s:?}");
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            error!("panic occurred: {s:?}");
        } else {
            error!("panic occurred");
        }

        if let Some(location) = panic_info.location() {
            error!(
                "panic occurred in file '{}' at line {}",
                location.file(),
                location.line(),
            );
        } else {
            error!("panic occurred but can't get location information...");
        }
    }));

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

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
    let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    ));

    let context = Arc::new(Context {
        device,
        queue,
        memory_allocator,
        command_buffer_allocator,
    });

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
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    width: jint,
    height: jint,
    data: JByteBuffer,
) {
    let context = unsafe { Arc::from_raw(handle as *const Context) };

    let len = env.get_direct_buffer_capacity(&data).unwrap();

    let buffer = Buffer::new_slice::<u8>(
        context.memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo::default(),
        len as u64,
    )
    .expect("Failed to create buffer");

    /* Lock subbufer and copy the entire RAW data into it */
    buffer
        .write()
        .expect("Failed to lock subbuffer for writing")
        .copy_from_slice(unsafe {
            slice::from_raw_parts(env.get_direct_buffer_address(&data).unwrap(), len)
        });

    let image = Image::new(
        context.memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R16_UINT,
            extent: [width as u32, height as u32, 1],
            usage: ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )
    .unwrap();

    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
        context.command_buffer_allocator.clone(),
        context.queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    command_buffer_builder
        .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(buffer, image))
        .unwrap();

    let command_buffer = command_buffer_builder.build().unwrap();

    let future = sync::now(context.device.clone())
        .then_execute(context.queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    info!("Everything succeeded!");

    /* Avoid the inner value from dropping */
    let _ = Arc::into_raw(context);
}
