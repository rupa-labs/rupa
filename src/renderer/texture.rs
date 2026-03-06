
pub struct Texture { 
    pub texture: wgpu::Texture, 
    pub view: wgpu::TextureView, 
    pub sampler: wgpu::Sampler, 
    pub bind_group: wgpu::BindGroup 
}

impl Texture {
    pub fn from_image(
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        img: &image::DynamicImage, 
        label: Option<&str>, 
        layout: &wgpu::BindGroupLayout
    ) -> Self {
        let rgba = img.to_rgba8();
        let (width, height) = (img.width(), img.height());
        let size = wgpu::Extent3d { width, height, depth_or_array_layers: 1 };
        
        let texture = device.create_texture(&wgpu::TextureDescriptor { 
            label, 
            size, 
            mip_level_count: 1, 
            sample_count: 1, 
            dimension: wgpu::TextureDimension::D2, 
            format: wgpu::TextureFormat::Rgba8UnormSrgb, 
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[] 
        });

        queue.write_texture(
            texture.as_image_copy(),
            &rgba,
            wgpu::TexelCopyBufferLayout { 
                offset: 0, 
                bytes_per_row: Some(4 * width), 
                rows_per_image: Some(height) 
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor { 
            mag_filter: wgpu::FilterMode::Nearest, 
            min_filter: wgpu::FilterMode::Nearest, 
            ..Default::default() 
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { 
            layout, 
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) }, 
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) }
            ], 
            label: Some("diffuse_bind_group") 
        });

        Self { texture, view, sampler, bind_group }
    }
}
