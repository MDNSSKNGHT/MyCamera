use std::sync::Arc;

use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
    descriptor_set::{
        DescriptorSet, WriteDescriptorSet, allocator::StandardDescriptorSetAllocator,
    },
    device::Device,
    image::view::ImageView,
    pipeline::{Pipeline, PipelineBindPoint},
};

use crate::pipeline::utils::create_compute_pipeline_from_shader;

pub fn setup_and_dispatch(
    descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    device: Arc<Device>,
    command_buffer_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    group_counts: [u32; 3],
    rgba_image: Arc<ImageView>,
    quantized_image: Arc<ImageView>,
) {
    let compute_shader = cs::load(device.clone()).unwrap();
    let compute_pipeline = create_compute_pipeline_from_shader(device.clone(), compute_shader);

    let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();
    let set = DescriptorSet::new(
        descriptor_set_allocator.clone(),
        layout.clone(),
        [
            WriteDescriptorSet::image_view(0, rgba_image),
            WriteDescriptorSet::image_view(1, quantized_image),
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
        command_buffer_builder.dispatch(group_counts).unwrap();
    }
}

mod cs {
    vulkano_shaders::shader! {
        bytes: "spirv/finishing_4.spv"
    }
}
