pub struct WGPUBuffer {
  gpu_buffer: wgpu::Buffer,
  size: usize,
  stride: usize,
  usage: wgpu::BufferUsage,
}

fn create_buffer<T: 'static + Copy>(
  device: &wgpu::Device,
  value: &[T],
  usage: wgpu::BufferUsage,
) -> wgpu::Buffer {
  device
    .create_buffer_mapped(value.len(), usage)
    .fill_from_slice(value)
}

impl WGPUBuffer {
  pub fn new<T: 'static + Copy>(
    device: &wgpu::Device,
    value: &[T],
    usage: wgpu::BufferUsage,
  ) -> Self {
    use std::mem;
    Self {
      gpu_buffer: create_buffer(device, value, usage),
      size: value.len(),
      stride: mem::size_of::<T>(),
      usage,
    }
  }

  pub fn usage(&self) -> wgpu::BufferUsage{
    self.usage
  }

  pub fn update<T: 'static + Copy>(
    &mut self,
    device: &wgpu::Device,
    encoder: &mut wgpu::CommandEncoder,
    value: &[T],
  ) -> &Self {
    assert_eq!(self.size, value.len());

    let new_gpu = create_buffer(device, value, wgpu::BufferUsage::COPY_SRC);
    encoder.copy_buffer_to_buffer(
      &new_gpu,
      0,
      &self.gpu_buffer,
      0,
      self.get_byte_length() as u64,
    );
    self
  }

  pub fn get_byte_length(&self) -> usize {
    self.size * self.stride
  }

  pub fn get_gpu_buffer(&self) -> &wgpu::Buffer {
    &self.gpu_buffer
  }
}
