use std::sync::Arc;

use vulkano::{
    buffer::Subbuffer,
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
    descriptor_set::DescriptorSet,
    image::view::ImageView,
    pipeline::ComputePipeline,
};

use crate::pipeline::context;

pub struct StageResources {
    pub compute_pipeline: Arc<ComputePipeline>,
    pub descriptor_set: Arc<DescriptorSet>,

    pub image_views: Vec<Arc<ImageView>>,
    pub buffers: Vec<Subbuffer<[u8]>>,
    pub commands: Vec<Arc<PrimaryAutoCommandBuffer>>,
}

#[derive(Default)]
pub struct StageOutput {
    pub image_views: Vec<Arc<ImageView>>,
    pub buffers: Vec<Subbuffer<[u8]>>,
    pub commands: Vec<Arc<PrimaryAutoCommandBuffer>>,
}

pub trait StageInPipeline {
    // In a pipeline the output of a stage is the input of the next stage
    fn create_stage_resources(
        &self,
        context: &context::Context,
        input: Option<StageOutput>,
    ) -> StageResources;

    fn bind_stage_pipeline_and_dispatch(
        &self,
        command_buffer_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        resources: &StageResources,
        work_groups: [u32; 3],
    );
}
