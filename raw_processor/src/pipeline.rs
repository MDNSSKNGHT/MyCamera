use std::sync::Arc;

use vulkano::{
    buffer::BufferContents,
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
    descriptor_set::{
        CopyDescriptorSet, DescriptorSet, DescriptorSetsCollection, WriteDescriptorSet,
        allocator::StandardDescriptorSetAllocator,
    },
    device::Device,
    pipeline::{
        ComputePipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo, compute::ComputePipelineCreateInfo,
        layout::PipelineDescriptorSetLayoutCreateInfo,
    },
    shader::ShaderModule,
};

pub fn create_compute_pipeline_from(
    device: Arc<Device>,
    shader_module: Arc<ShaderModule>,
) -> Arc<ComputePipeline> {
    let compute_shader = shader_module.entry_point("main").unwrap();
    let stage = PipelineShaderStageCreateInfo::new(compute_shader);
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

    /* I don't know how to return as Result<> */
    compute_pipeline
}

pub fn create_descriptor_sets(
    descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    compute_pipeline: Arc<ComputePipeline>,
    descriptor_writes: impl IntoIterator<Item = WriteDescriptorSet>,
    descriptor_copies: impl IntoIterator<Item = CopyDescriptorSet>,
) -> Arc<DescriptorSet> {
    let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();
    let set = DescriptorSet::new(
        descriptor_set_allocator.clone(),
        layout.clone(),
        descriptor_writes,
        descriptor_copies,
    )
    .unwrap();

    set
}

// pub fn bind_and_dispatch_pipeline(
//     mut command_buffer_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
//     compute_pipeline: Arc<ComputePipeline>,
//     descriptor_sets: impl DescriptorSetsCollection,
//     group_counts: [u32; 3],
// ) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> {
//     command_buffer_builder
//         .bind_pipeline_compute(compute_pipeline.clone())
//         .unwrap()
//         .bind_descriptor_sets(
//             PipelineBindPoint::Compute,
//             compute_pipeline.layout().clone(),
//             0,
//             descriptor_sets,
//         )
//         .unwrap();

//     unsafe {
//         command_buffer_builder.dispatch(group_counts).unwrap();
//     }

//     command_buffer_builder
// }

pub fn bind_and_dispatch_pipeline_with_constants<Pc>(
    mut command_buffer_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    compute_pipeline: Arc<ComputePipeline>,
    constants: Pc,
    descriptor_sets: impl DescriptorSetsCollection,
    group_counts: [u32; 3],
) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>
where
    Pc: BufferContents,
{
    command_buffer_builder
        .bind_pipeline_compute(compute_pipeline.clone())
        .unwrap()
        .push_constants(compute_pipeline.layout().clone(), 0, constants)
        .unwrap()
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            compute_pipeline.layout().clone(),
            0,
            descriptor_sets,
        )
        .unwrap();

    unsafe {
        command_buffer_builder.dispatch(group_counts).unwrap();
    }

    command_buffer_builder
}
