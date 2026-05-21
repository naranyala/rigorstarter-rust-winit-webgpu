pub struct ShaderModule {
    pub inner: wgpu::ShaderModule,
}

impl ShaderModule {
    pub fn from_wgsl(device: &wgpu::Device, label: &str, source: &str) -> Self {
        let inner = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        Self { inner }
    }

    pub fn from_wgsl_include(
        device: &wgpu::Device,
        label: &str,
        source: &'static str,
    ) -> Self {
        let inner = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        Self { inner }
    }
}

pub struct RenderPipelineBuilder {
    label: String,
    shader: Option<ShaderModule>,
    vertex_entry: String,
    fragment_entry: String,
    format: wgpu::TextureFormat,
    buffers: Vec<wgpu::VertexBufferLayout<'static>>,
    topology: wgpu::PrimitiveTopology,
    blend: Option<wgpu::BlendState>,
}

impl RenderPipelineBuilder {
    pub fn new(label: &str, format: wgpu::TextureFormat) -> Self {
        Self {
            label: label.to_string(),
            shader: None,
            vertex_entry: "vs_main".to_string(),
            fragment_entry: "fs_main".to_string(),
            format,
            buffers: Vec::new(),
            topology: wgpu::PrimitiveTopology::TriangleList,
            blend: Some(wgpu::BlendState::REPLACE),
        }
    }

    pub fn shader(mut self, shader: ShaderModule) -> Self {
        self.shader = Some(shader);
        self
    }

    pub fn vertex_entry(mut self, entry: &str) -> Self {
        self.vertex_entry = entry.to_string();
        self
    }

    pub fn fragment_entry(mut self, entry: &str) -> Self {
        self.fragment_entry = entry.to_string();
        self
    }

    pub fn vertex_buffer(mut self, layout: wgpu::VertexBufferLayout<'static>) -> Self {
        self.buffers.push(layout);
        self
    }

    pub fn topology(mut self, topology: wgpu::PrimitiveTopology) -> Self {
        self.topology = topology;
        self
    }

    pub fn blend(mut self, blend: wgpu::BlendState) -> Self {
        self.blend = Some(blend);
        self
    }

    pub fn build(self, device: &wgpu::Device, bind_groups: &[&wgpu::BindGroupLayout]) -> RenderPipeline {
        let shader = self.shader.expect("Shader must be set");

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("{} Layout", self.label)),
            bind_group_layouts: bind_groups,
            push_constant_ranges: &[],
        });

        let inner = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&self.label),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader.inner,
                entry_point: Some(&self.vertex_entry),
                buffers: &self.buffers,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader.inner,
                entry_point: Some(&self.fragment_entry),
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.format,
                    blend: self.blend,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: self.topology,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        RenderPipeline { inner }
    }
}

pub struct RenderPipeline {
    pub inner: wgpu::RenderPipeline,
}
