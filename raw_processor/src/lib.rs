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
        AutoCommandBufferBuilder, CommandBufferUsage,
        allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
    },
    descriptor_set::{WriteDescriptorSet, allocator::StandardDescriptorSetAllocator},
    device::{
        Device, DeviceCreateInfo, DeviceExtensions, DeviceFeatures, Queue, QueueCreateInfo,
        QueueFlags,
    },
    format::Format,
    image::{Image, ImageCreateInfo, ImageType, ImageUsage, view::ImageView},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    sync::{self, GpuFuture},
};

mod params;
mod pipeline;
mod shader;

struct Context {
    device: Arc<Device>,
    queue: Arc<Queue>,
    memory_allocator: Arc<StandardMemoryAllocator>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
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
        .find(|physical_device| {
            physical_device
                .supported_extensions()
                .khr_shader_float16_int8
        })
        .expect("Failed to find suitable physical device");

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
            enabled_extensions: DeviceExtensions {
                khr_16bit_storage: true,
                khr_shader_float16_int8: true,
                ..Default::default()
            },
            enabled_features: DeviceFeatures {
                storage_buffer16_bit_access: true,
                shader_int16: true,
                ..Default::default()
            },
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
    let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
        device.clone(),
        Default::default(),
    ));

    let context = Arc::new(Context {
        device,
        queue,
        memory_allocator,
        command_buffer_allocator,
        descriptor_set_allocator,
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
            usage: BufferUsage::TRANSFER_SRC | BufferUsage::STORAGE_BUFFER,
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
            format: Format::R32_SFLOAT,
            extent: [width as u32, height as u32, 1],
            usage: ImageUsage::STORAGE,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )
    .unwrap();

    let image_view = ImageView::new_default(image.clone()).unwrap();

    let finishing_1 =
        shader::finishing_1::load(context.device.clone()).expect("Failed to create shader module");

    // let compute_shader = finishing_1.entry_point("main").unwrap();
    // let stage = PipelineShaderStageCreateInfo::new(compute_shader);
    // let layout = PipelineLayout::new(
    //     context.device.clone(),
    //     PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
    //         .into_pipeline_layout_create_info(context.device.clone())
    //         .unwrap(),
    // )
    // .unwrap();

    // let compute_pipeline = ComputePipeline::new(
    //     context.device.clone(),
    //     None,
    //     ComputePipelineCreateInfo::stage_layout(stage, layout),
    // )
    // .expect("Failed to create compute pipeline");

    let compute_pipeline =
        pipeline::create_compute_pipeline_from(context.device.clone(), finishing_1);

    let descriptor_set = pipeline::create_descriptor_sets(
        context.descriptor_set_allocator.clone(),
        compute_pipeline.clone(),
        [
            WriteDescriptorSet::buffer(0, buffer.clone()),
            WriteDescriptorSet::image_view(1, image_view.clone()),
        ],
        [],
    );

    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
        context.command_buffer_allocator.clone(),
        context.queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    command_buffer_builder = pipeline::bind_and_dispatch_pipeline_with_constants(
        command_buffer_builder,
        compute_pipeline,
        params::Stage1Parameters {
            stride: width as u32,
            white_level: 1023,
            black_level: 0,
        },
        descriptor_set,
        [width as u32 / 4, height as u32 / 4, 1],
    );

    let rgb = Image::new(
        context.memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R32G32B32A32_SFLOAT,
            extent: [width as u32, height as u32, 1],
            usage: ImageUsage::STORAGE,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )
    .unwrap();

    let rgb_view = ImageView::new_default(rgb.clone()).unwrap();

    let finishing_2 =
        shader::finishing_2::load(context.device.clone()).expect("Failed to create shader module");

    let compute_pipeline =
        pipeline::create_compute_pipeline_from(context.device.clone(), finishing_2);

    let descriptor_set = pipeline::create_descriptor_sets(
        context.descriptor_set_allocator.clone(),
        compute_pipeline.clone(),
        [
            WriteDescriptorSet::image_view(0, image_view),
            WriteDescriptorSet::image_view(1, rgb_view),
        ],
        [],
    );

    command_buffer_builder = pipeline::bind_and_dispatch_pipeline_with_constants(
        command_buffer_builder,
        compute_pipeline,
        params::Stage2Parameters {
            width: width as u32,
            height: height as u32,
        },
        descriptor_set,
        [width as u32 / 4, height as u32 / 4, 1],
    );

    // command_buffer_builder
    //     .bind_pipeline_compute(compute_pipeline.clone())
    //     .unwrap()
    //     .push_constants(compute_pipeline.layout().clone(), 0, constants)
    //     .unwrap()
    //     .bind_descriptor_sets(
    //         PipelineBindPoint::Compute,
    //         compute_pipeline.layout().clone(),
    //         0,
    //         set,
    //     )
    //     .unwrap();

    // unsafe {
    //     command_buffer_builder
    //         .dispatch([width as u32 / 4, height as u32 / 4, 1])
    //         .unwrap();
    // }

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
