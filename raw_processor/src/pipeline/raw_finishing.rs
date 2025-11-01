use std::{slice, sync::Arc};

use vulkano::{
    DeviceSize,
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferReadGuard, BufferUsage, Subbuffer},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferToImageInfo, CopyImageToBufferInfo,
        PrimaryAutoCommandBuffer,
    },
    descriptor_set::{DescriptorSet, WriteDescriptorSet},
    device::Device,
    format::Format,
    image::{Image, ImageCreateInfo, ImageUsage, view::ImageView},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
    pipeline::{
        ComputePipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo, compute::ComputePipelineCreateInfo,
        layout::PipelineDescriptorSetLayoutCreateInfo,
    },
    shader::ShaderModule,
    sync::{self, GpuFuture},
};

pub struct RawFinishing {
    image_views: [Arc<ImageView>; 4],
    buffers: [Subbuffer<[u8]>; 1],
    copy_commands: [Arc<PrimaryAutoCommandBuffer>; 2],
    work_groups: [u32; 3],
}

impl RawFinishing {
    pub fn new(
        context: &crate::pipeline::Context,
        data: *const u8,
        len: usize,
        size: [i32; 2],
    ) -> RawFinishing {
        let extent = [size[0] as u32, size[1] as u32, 1];

        let (_, raw_image_view, copy_buffer_to_raw_image) = {
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
                .copy_from_slice(unsafe { slice::from_raw_parts(data, len as usize) });

            let image = Image::new(
                context.memory_allocator.clone(),
                ImageCreateInfo {
                    format: Format::R16_UINT,
                    extent,
                    usage: ImageUsage::STORAGE | ImageUsage::TRANSFER_DST,
                    ..Default::default()
                },
                AllocationCreateInfo::default(),
            )
            .unwrap();

            let view = ImageView::new_default(image.clone()).unwrap();

            let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
                context.command_buffer_allocator.clone(),
                context.queue.queue_family_index(),
                CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap();

            command_buffer_builder
                .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(buffer, image.clone()))
                .unwrap();

            let command_buffer = command_buffer_builder.build().unwrap();

            (image, view, command_buffer)
        };

        let (_, raw_normalized_image_view) = {
            let image = Image::new(
                context.memory_allocator.clone(),
                ImageCreateInfo {
                    format: Format::R16_SFLOAT,
                    extent,
                    usage: ImageUsage::STORAGE,
                    ..Default::default()
                },
                AllocationCreateInfo::default(),
            )
            .unwrap();

            let view = ImageView::new_default(image.clone()).unwrap();

            (image, view)
        };

        let (_, rgba_intermediate_image_view) = {
            let image = Image::new(
                context.memory_allocator.clone(),
                ImageCreateInfo {
                    format: Format::R16G16B16A16_SFLOAT,
                    extent,
                    usage: ImageUsage::STORAGE,
                    ..Default::default()
                },
                AllocationCreateInfo::default(),
            )
            .unwrap();

            let view = ImageView::new_default(image.clone()).unwrap();

            (image, view)
        };

        let (_, rgba_quantized_image_view, rgba_quantized_buffer, copy_quantized_image_to_buffer) = {
            let image = Image::new(
                context.memory_allocator.clone(),
                ImageCreateInfo {
                    format: Format::R8G8B8A8_UNORM,
                    extent,
                    usage: ImageUsage::STORAGE | ImageUsage::TRANSFER_SRC,
                    ..Default::default()
                },
                AllocationCreateInfo::default(),
            )
            .unwrap();

            let view = ImageView::new_default(image.clone()).unwrap();

            let buffer = Buffer::from_iter(
                context.memory_allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::TRANSFER_DST,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_HOST
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                (0..size[0] * size[1] * 4).map(|_| 0u8),
            )
            .unwrap();

            let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
                context.command_buffer_allocator.clone(),
                context.queue.queue_family_index(),
                CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap();

            command_buffer_builder
                .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
                    image.clone(),
                    buffer.clone(),
                ))
                .unwrap();

            let command_buffer = command_buffer_builder.build().unwrap();

            (image, view, buffer, command_buffer)
        };

        let work_groups = {
            let w = size[0] as u32;
            let h = size[1] as u32;
            // Rounding up
            [(w + 7) / 8, (h + 7) / 8, 1]
        };

        RawFinishing {
            image_views: [
                raw_image_view,
                raw_normalized_image_view,
                rgba_intermediate_image_view,
                rgba_quantized_image_view,
            ],
            buffers: [rgba_quantized_buffer],
            copy_commands: [copy_buffer_to_raw_image, copy_quantized_image_to_buffer],
            work_groups,
        }
    }

    pub fn process(&self, context: &crate::pipeline::Context) {
        let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
            context.command_buffer_allocator.clone(),
            context.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        {
            let compute_shader = finishing_1::load(context.device.clone()).unwrap();
            let compute_pipeline =
                create_compute_pipeline_from_shader(context.device.clone(), compute_shader);

            let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();
            let set = DescriptorSet::new(
                context.descriptor_set_allocator.clone(),
                layout.clone(),
                [
                    WriteDescriptorSet::image_view(0, self.image_views[0].clone() /* raw */),
                    WriteDescriptorSet::image_view(
                        1,
                        self.image_views[1].clone(), /* raw_normalized */
                    ),
                ],
                [],
            )
            .unwrap();

            #[derive(BufferContents)]
            #[repr(C)]
            struct Parameters {
                white_level: u32,
                black_level: u32,
            }

            let parameters = Parameters {
                white_level: 1023,
                black_level: 0,
            };

            command_buffer_builder
                .bind_pipeline_compute(compute_pipeline.clone())
                .unwrap()
                .push_constants(compute_pipeline.layout().clone(), 0, parameters)
                .unwrap()
                .bind_descriptor_sets(
                    PipelineBindPoint::Compute,
                    compute_pipeline.layout().clone(),
                    0,
                    set,
                )
                .unwrap();

            unsafe {
                command_buffer_builder.dispatch(self.work_groups).unwrap();
            }
        }

        {
            let compute_shader = finishing_2::load(context.device.clone()).unwrap();
            let compute_pipeline =
                create_compute_pipeline_from_shader(context.device.clone(), compute_shader);

            let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();
            let set = DescriptorSet::new(
                context.descriptor_set_allocator.clone(),
                layout.clone(),
                [
                    WriteDescriptorSet::image_view(
                        0,
                        self.image_views[1].clone(), /* raw_normalized */
                    ),
                    WriteDescriptorSet::image_view(1, self.image_views[2].clone() /* rgba */),
                ],
                [],
            )
            .unwrap();

            #[derive(BufferContents)]
            #[repr(C)]
            struct Parameters {
                cfa: [u32; 4],
            }

            let parameters = Parameters { cfa: [0, 1, 2, 3] };

            command_buffer_builder
                .bind_pipeline_compute(compute_pipeline.clone())
                .unwrap()
                .push_constants(compute_pipeline.layout().clone(), 0, parameters)
                .unwrap()
                .bind_descriptor_sets(
                    PipelineBindPoint::Compute,
                    compute_pipeline.layout().clone(),
                    0,
                    set,
                )
                .unwrap();

            unsafe {
                command_buffer_builder.dispatch(self.work_groups).unwrap();
            }
        }

        {
            let compute_shader = finishing_3::load(context.device.clone()).unwrap();
            let compute_pipeline =
                create_compute_pipeline_from_shader(context.device.clone(), compute_shader);

            let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();
            let set = DescriptorSet::new(
                context.descriptor_set_allocator.clone(),
                layout.clone(),
                [WriteDescriptorSet::image_view(
                    0,
                    self.image_views[2].clone(), /* rgba */
                )],
                [],
            )
            .unwrap();

            #[derive(BufferContents)]
            #[repr(C)]
            struct Parameters {
                forward_matrix_1: [[f32; 4]; 3],
                forward_matrix_2: [[f32; 4]; 3],
            }

            let parameters = Parameters {
                forward_matrix_1: [
                    [1.0, 0.0, 0.0, 0.0 /* padding */],
                    [0.0, 1.0, 0.0, 0.0 /* padding */],
                    [0.0, 0.0, 1.0, 0.0 /* padding */],
                ], /* TODO: pass actual value */
                forward_matrix_2: [
                    [1.0, 0.0, 0.0, 0.0 /* padding */],
                    [0.0, 1.0, 0.0, 0.0 /* padding */],
                    [0.0, 0.0, 1.0, 0.0 /* padding */],
                ], /* TODO: pass actual value */
            };

            command_buffer_builder
                .bind_pipeline_compute(compute_pipeline.clone())
                .unwrap()
                .push_constants(compute_pipeline.layout().clone(), 0, parameters)
                .unwrap()
                .bind_descriptor_sets(
                    PipelineBindPoint::Compute,
                    compute_pipeline.layout().clone(),
                    0,
                    set,
                )
                .unwrap();

            unsafe {
                command_buffer_builder.dispatch(self.work_groups).unwrap();
            }
        }

        {
            let compute_shader = finishing_4::load(context.device.clone()).unwrap();
            let compute_pipeline =
                create_compute_pipeline_from_shader(context.device.clone(), compute_shader);

            let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();
            let set = DescriptorSet::new(
                context.descriptor_set_allocator.clone(),
                layout.clone(),
                [
                    WriteDescriptorSet::image_view(0, self.image_views[2].clone() /* rgba */),
                    WriteDescriptorSet::image_view(
                        1,
                        self.image_views[3].clone(), /* rgba_quantized */
                    ),
                ],
                [],
            )
            .unwrap();

            command_buffer_builder
                .bind_pipeline_compute(compute_pipeline.clone())
                .unwrap()
                .bind_descriptor_sets(
                    PipelineBindPoint::Compute,
                    compute_pipeline.layout().clone(),
                    0,
                    set,
                )
                .unwrap();

            unsafe {
                command_buffer_builder.dispatch(self.work_groups).unwrap();
            }
        }

        let command_buffer = command_buffer_builder.build().unwrap();

        sync::now(context.device.clone())
            .then_execute(
                context.queue.clone(),
                self.copy_commands[0].clone(), /* buffer to raw_image */
            )
            .unwrap()
            .then_execute(context.queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();

        sync::now(context.device.clone())
            .then_execute(
                context.queue.clone(),
                self.copy_commands[1].clone(), /* rgba_quantized to buffer */
            )
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();

        // info!("Command buffer execution succeeded");
    }

    pub fn read_output_buffer(&self) -> BufferReadGuard<'_, [u8]> {
        self.buffers[0].read().unwrap() /* rgba quantized buffer */
    }
}

fn create_compute_pipeline_from_shader(
    device: Arc<Device>,
    shader_module: Arc<ShaderModule>,
) -> Arc<ComputePipeline> {
    let stage = PipelineShaderStageCreateInfo::new(shader_module.entry_point("main").unwrap());
    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
    )
    .unwrap();

    let compute_pipeline = ComputePipeline::new(
        device.clone(),
        None,
        ComputePipelineCreateInfo::stage_layout(stage, layout),
    )
    .expect("Failed to create compute pipeline");

    compute_pipeline
}

mod finishing_1 {
    vulkano_shaders::shader! {
        bytes: "spirv/finishing_1.spv"
    }
}

mod finishing_2 {
    vulkano_shaders::shader! {
        bytes: "spirv/finishing_2.spv"
    }
}

mod finishing_3 {
    vulkano_shaders::shader! {
        bytes: "spirv/finishing_3.spv"
    }
}

mod finishing_4 {
    vulkano_shaders::shader! {
        bytes: "spirv/finishing_4.spv"
    }
}
