use std::{panic, slice, sync::Arc};

use android_logger::Config;
use jni::{
    JNIEnv,
    objects::{JByteBuffer, JClass},
    sys::{jint, jlong},
};
use log::{LevelFilter, error, info};
use vulkano::{
    DeviceSize, VulkanLibrary,
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferToImageInfo,
        allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
    },
    device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags},
    format::Format,
    image::{Image, ImageCreateInfo, ImageUsage},
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

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
    // let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
    //     device.clone(),
    //     Default::default(),
    // ));
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

    let (_, cb_copy_to_raw_image) = {
        let len = env.get_direct_buffer_capacity(&data).unwrap();

        let buffer = Buffer::new_slice::<u8>(
            context.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            len as DeviceSize,
        )
        .unwrap();

        // Lock subbufer and copy the entire RAW data into it
        buffer
            .write()
            .expect("Failed to lock subbufer for writing")
            .copy_from_slice(unsafe {
                slice::from_raw_parts(env.get_direct_buffer_address(&data).unwrap(), len)
            });

        let image = Image::new(
            context.memory_allocator.clone(),
            ImageCreateInfo {
                format: Format::R16_UINT,
                extent: [width as u32, height as u32, 1],
                usage: ImageUsage::STORAGE | ImageUsage::TRANSFER_DST,
                ..Default::default()
            },
            AllocationCreateInfo::default(),
        )
        .unwrap();

        let mut cbb = AutoCommandBufferBuilder::primary(
            context.command_buffer_allocator.clone(),
            context.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        cbb.copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(buffer, image.clone()))
            .unwrap();

        let cb = cbb.build().unwrap();

        // cb.execute(context.queue.clone())
        //     .unwrap()
        //     .then_signal_fence_and_flush()
        //     .unwrap()
        //     .wait(None /* timeout */)
        //     .unwrap();

        (image, cb)
    };

    // We're executing the copy command here because in the future we will chain
    // multiple `GPUFuture`s
    sync::now(context.device.clone())
        .then_execute(context.queue.clone(), cb_copy_to_raw_image)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    info!("Command buffer execution succeeded");

    // info!("handle: {}", x);
    // info!("width: {}, height: {}", width, height);

    // Avoid the inner value from dropping
    let _ = Arc::into_raw(context);
}
