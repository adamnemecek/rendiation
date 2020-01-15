use crate::geometry::StandardGeometry;
use rendiation::*;

pub struct TexShading {
  pipeline: WGPUPipeline,

  bindgroup: WGPUBindGroup,
}

impl TexShading {
  pub fn new(
    renderer: &WGPURenderer,
    texture: &WGPUTexture,
    matrix_buffer: &WGPUBuffer,
    sampler: &WGPUSampler,
  ) -> Self {
    let mut pipeline_builder = WGPUPipelineDescriptorBuilder::new();
    pipeline_builder
      .vertex_shader(include_str!("./shader.vert"))
      .frag_shader(include_str!("./shader.frag"))
      .binding_group(
        BindGroupLayoutBuilder::new()
          .binding(wgpu::BindGroupLayoutBinding {
            binding: 0,
            visibility: wgpu::ShaderStage::VERTEX,
            ty: wgpu::BindingType::UniformBuffer { dynamic: false },
          })
          .binding(wgpu::BindGroupLayoutBinding {
            binding: 1,
            visibility: wgpu::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::SampledTexture {
              multisampled: false,
              dimension: wgpu::TextureViewDimension::D2,
            },
          })
          .binding(wgpu::BindGroupLayoutBinding {
            binding: 2,
            visibility: wgpu::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::Sampler,
          }),
      );

    let pipeline =
      pipeline_builder.build::<StandardGeometry>(&renderer.device, &renderer.swap_chain_descriptor);

    let texture_view = texture.make_default_view();
    let bindgroup = BindGroupBuilder::new()
      .buffer(matrix_buffer)
      .texture(&texture_view)
      .sampler(&sampler)
      .build(&renderer.device, &pipeline.bind_group_layouts[0]);

    TexShading {
      pipeline,
      bindgroup,
    }
  }

  pub fn use_shading(&self, pass: &mut WGPURenderPass){
    pass.gpu_pass.set_pipeline(&self.pipeline.pipeline);
    pass.gpu_pass.set_bind_group(0, &self.bindgroup.gpu_bindgroup, &[]);
  }
}
