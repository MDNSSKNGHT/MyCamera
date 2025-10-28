use std::sync::Arc;

use vulkano::{
    device::Device,
    pipeline::{
        ComputePipeline, PipelineLayout, PipelineShaderStageCreateInfo,
        compute::ComputePipelineCreateInfo, layout::PipelineDescriptorSetLayoutCreateInfo,
    },
    shader::ShaderModule,
};

pub fn create_compute_pipeline_from_shader(
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
