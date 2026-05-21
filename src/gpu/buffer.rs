use wgpu::util::DeviceExt;

pub struct Buffer {
    pub inner: wgpu::Buffer,
    pub size: u64,
}

impl Buffer {
    pub fn new_uniform(device: &wgpu::Device, label: &str, data: &[u8]) -> Self {
        let inner = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: data,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let size = inner.size();
        Self { inner, size }
    }

    pub fn new_storage(device: &wgpu::Device, label: &str, size: u64) -> Self {
        let inner = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self { inner, size }
    }

    pub fn new_vertex(device: &wgpu::Device, label: &str, data: &[u8]) -> Self {
        let inner = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: data,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let size = inner.size();
        Self { inner, size }
    }

    pub fn new_index(device: &wgpu::Device, label: &str, data: &[u8]) -> Self {
        let inner = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: data,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });
        let size = inner.size();
        Self { inner, size }
    }

    pub fn write_data(&self, queue: &wgpu::Queue, data: &[u8]) {
        queue.write_buffer(&self.inner, 0, data);
    }
}
