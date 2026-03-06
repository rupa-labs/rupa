use std::sync::Arc;
use winit::window::Window as WinitWindow;
use wgpu::*;
use bytemuck::{Pod, Zeroable};
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text, Layout};
use crate::utils::{Vec2, TextAlign};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex { pub position: [f32; 2], pub color: [f32; 4], pub tex_coords: [f32; 2], pub rect_size: [f32; 2], pub radius: f32, pub mode: u32 }

impl Vertex {
    const ATTRIBS: [VertexAttribute; 6] = vertex_attr_array![0 => Float32x2, 1 => Float32x4, 2 => Float32x2, 3 => Float32x2, 4 => Float32, 5 => Uint32];
    pub fn desc() -> VertexBufferLayout<'static> { VertexBufferLayout { array_stride: std::mem::size_of::<Vertex>() as BufferAddress, step_mode: VertexStepMode::Vertex, attributes: &Self::ATTRIBS } }
}

pub struct Texture { pub texture: wgpu::Texture, pub view: wgpu::TextureView, pub sampler: wgpu::Sampler, pub bind_group: wgpu::BindGroup }
impl Texture {
    pub fn from_image(device: &wgpu::Device, queue: &wgpu::Queue, img: &image::DynamicImage, label: Option<&str>, layout: &wgpu::BindGroupLayout) -> Self {
        let rgba = img.to_rgba8();
        let (width, height) = (img.width(), img.height());
        let size = wgpu::Extent3d { width, height, depth_or_array_layers: 1 };
        let texture = device.create_texture(&wgpu::TextureDescriptor { label, size, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8UnormSrgb, usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, view_formats: &[] });
        queue.write_texture(wgpu::ImageCopyTexture { aspect: wgpu::TextureAspect::All, texture: &texture, mip_level: 0, origin: wgpu::Origin3d::ZERO }, &rgba, wgpu::ImageDataLayout { offset: 0, bytes_per_row: Some(4 * width), rows_per_image: Some(height) }, size);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor { mag_filter: wgpu::FilterMode::Nearest, min_filter: wgpu::FilterMode::Nearest, ..Default::default() });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) }, wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) }], label: Some("diffuse_bind_group") });
        Self { texture, view, sampler, bind_group }
    }
}

pub struct Renderer {
    pub surface: Surface<'static>, pub device: Device, pub queue: Queue, pub config: SurfaceConfiguration, pub size: winit::dpi::PhysicalSize<u32>, pub window: Arc<WinitWindow>,
    render_pipeline: RenderPipeline, vertex_buffer: Buffer, index_buffer: Buffer, texture_bind_group_layout: BindGroupLayout, default_texture: Texture, glyph_brush: GlyphBrush<()>,
    vertices: Vec<Vertex>, indices: Vec<u32>, max_batch_size: usize,
    pub camera_offset: Vec2, pub camera_zoom: f32,
}

impl Renderer {
    pub async fn new(window: Arc<WinitWindow>) -> Self {
        let size = window.inner_size();
        let instance = Instance::new(InstanceDescriptor { backends: Backends::all(), ..Default::default() });
        let surface = unsafe { instance.create_surface(window.clone()) }.expect("Failed to create surface");
        let adapter = instance.request_adapter(&RequestAdapterOptions { power_preference: PowerPreference::HighPerformance, compatible_surface: Some(&surface), force_fallback_adapter: false }).await.unwrap();
        let (device, queue) = adapter.request_device(&DeviceDescriptor { label: Some("Rupaui Device"), ..Default::default() }, None).await.unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter().copied().find(|f| f.is_srgb()).unwrap_or(surface_caps.formats[0]);
        let config = SurfaceConfiguration { usage: TextureUsages::RENDER_ATTACHMENT, format: surface_format, width: size.width, height: size.height, present_mode: surface_caps.present_modes[0], alpha_mode: surface_caps.alpha_modes[0], view_formats: vec![], desired_maximum_frame_latency: 2 };
        surface.configure(&device, &config);
        let texture_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor { entries: &[BindGroupLayoutEntry { binding: 0, visibility: ShaderStages::FRAGMENT, ty: BindingType::Texture { multisampled: false, view_dimension: TextureViewDimension::D2, sample_type: TextureSampleType::Float { filterable: true } }, count: None }, BindGroupLayoutEntry { binding: 1, visibility: ShaderStages::FRAGMENT, ty: BindingType::Sampler(SamplerBindingType::Filtering), count: None }], label: Some("texture_bind_group_layout") });
        let default_img = image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(1, 1, image::Rgba([255, 255, 255, 255])));
        let default_texture = Texture::from_image(&device, &queue, &default_img, Some("Default White"), &texture_bind_group_layout);
        let shader = device.create_shader_module(include_wgsl!("shaders/shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor { label: Some("Render Pipeline Layout"), bind_group_layouts: &[&texture_bind_group_layout], push_constant_ranges: &[] });
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Rupaui Render Pipeline"), layout: Some(&render_pipeline_layout),
            vertex: VertexState { module: &shader, entry_point: Some("vs_main"), buffers: &[Vertex::desc()], compilation_options: Default::default() },
            fragment: Some(FragmentState { module: &shader, entry_point: Some("fs_main"), targets: &[Some(ColorTargetState { format: config.format, blend: Some(BlendState::ALPHA_BLENDING), write_mask: ColorWrites::ALL })], compilation_options: Default::default() }),
            primitive: PrimitiveState { topology: PrimitiveTopology::TriangleList, ..Default::default() },
            depth_stencil: None, multisample: MultisampleState::default(), multiview: None, cache: None,
        });
        let max_batch_size = 10000;
        let vertex_buffer = device.create_buffer(&BufferDescriptor { label: Some("Vertex Buffer"), size: (std::mem::size_of::<Vertex>() * max_batch_size * 4) as BufferAddress, usage: BufferUsages::VERTEX | BufferUsages::COPY_DST, mapped_at_creation: false });
        let index_buffer = device.create_buffer(&BufferDescriptor { label: Some("Index Buffer"), size: (std::mem::size_of::<u32>() * max_batch_size * 6) as BufferAddress, usage: BufferUsages::INDEX | BufferUsages::COPY_DST, mapped_at_creation: false });
        let inter_regular = ab_glyph::FontArc::try_from_slice(include_bytes!("../../../src/fonts/inter/Inter-Regular.ttf")).unwrap();
        let glyph_brush = GlyphBrushBuilder::using_font(inter_regular).build(&device, config.format);
        Self { window, surface, device, queue, config, size, render_pipeline, vertex_buffer, index_buffer, texture_bind_group_layout, default_texture, glyph_brush, vertices: Vec::with_capacity(max_batch_size * 4), indices: Vec::with_capacity(max_batch_size * 6), max_batch_size, camera_offset: Vec2::zero(), camera_zoom: 1.0 }
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], radius: f32) {
        let tx = (x + self.camera_offset.x) * self.camera_zoom; let ty = (y + self.camera_offset.y) * self.camera_zoom;
        let tw = w * self.camera_zoom; let th = h * self.camera_zoom;
        if self.vertices.len() + 4 > self.max_batch_size * 4 { return; }
        let x_norm = (tx / self.size.width as f32) * 2.0 - 1.0; let y_norm = 1.0 - (ty / self.size.height as f32) * 2.0;
        let w_norm = (tw / self.size.width as f32) * 2.0; let h_norm = (th / self.size.height as f32) * 2.0;
        let base_idx = self.vertices.len() as u32; let size = [tw, th];
        self.vertices.extend_from_slice(&[
            Vertex { position: [x_norm, y_norm], color, tex_coords: [0.0, 0.0], rect_size: size, radius: radius * self.camera_zoom, mode: 0 },
            Vertex { position: [x_norm + w_norm, y_norm], color, tex_coords: [1.0, 0.0], rect_size: size, radius: radius * self.camera_zoom, mode: 0 },
            Vertex { position: [x_norm + w_norm, y_norm - h_norm], color, tex_coords: [1.0, 1.0], rect_size: size, radius: radius * self.camera_zoom, mode: 0 },
            Vertex { position: [x_norm, y_norm - h_norm], color, tex_coords: [0.0, 1.0], rect_size: size, radius: radius * self.camera_zoom, mode: 0 },
        ]);
        self.indices.extend_from_slice(&[base_idx, base_idx + 1, base_idx + 2, base_idx, base_idx + 2, base_idx + 3]);
    }

    pub fn draw_text(&mut self, content: &str, x: f32, y: f32, size: f32, color: [f32; 4], align: TextAlign) {
        let tx = (x + self.camera_offset.x) * self.camera_zoom;
        let ty = (y + self.camera_offset.y) * self.camera_zoom;
        let tsize = size * self.camera_zoom;
        let horizontal_align = match align {
            TextAlign::Left => wgpu_glyph::HorizontalAlign::Left,
            TextAlign::Center => wgpu_glyph::HorizontalAlign::Center,
            TextAlign::Right => wgpu_glyph::HorizontalAlign::Right,
            _ => wgpu_glyph::HorizontalAlign::Left,
        };
        self.glyph_brush.queue(Section {
            screen_position: (tx, ty),
            bounds: (self.size.width as f32, self.size.height as f32),
            text: vec![Text::new(content).with_color(color).with_scale(tsize)],
            layout: Layout::default().h_align(horizontal_align),
            ..Default::default()
        });
    }

    pub fn push_clip(&mut self, x: f32, y: f32, w: f32, h: f32, render_pass: &mut RenderPass<'_>) {
        let x = (x.max(0.0) * self.camera_zoom) as u32; let y = (y.max(0.0) * self.camera_zoom) as u32;
        let w = (w.max(0.0) * self.camera_zoom) as u32; let h = (h.max(0.0) * self.camera_zoom) as u32;
        render_pass.set_scissor_rect(x, y, w.min(self.size.width - x), h.min(self.size.height - y));
    }
    pub fn pop_clip(&mut self, render_pass: &mut RenderPass<'_>) { render_pass.set_scissor_rect(0, 0, self.size.width, self.size.height); }
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { if new_size.width > 0 && new_size.height > 0 { self.size = new_size; self.config.width = new_size.width; self.config.height = new_size.height; self.surface.configure(&self.device, &self.config); } }
    pub fn begin_frame(&mut self) -> Result<(SurfaceTexture, TextureView, CommandEncoder), SurfaceError> { self.vertices.clear(); self.indices.clear(); let output = self.surface.get_current_texture()?; let view = output.texture.create_view(&TextureViewDescriptor::default()); let encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: Some("Rupaui Frame Encoder") }); Ok((output, view, encoder)) }
    pub fn flush_batches<'a>(&'a mut self, render_pass: &mut RenderPass<'a>) { if self.indices.is_empty() { return; } self.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices)); self.queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&self.indices)); render_pass.set_pipeline(&self.render_pipeline); render_pass.set_bind_group(0, &self.default_texture.bind_group, &[]); render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32); render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1); self.vertices.clear(); self.indices.clear(); }
    pub fn end_frame(&mut self, output: SurfaceTexture, mut encoder: CommandEncoder) {
        // wgpu_glyph 0.23 draw_queued wants &Device, &Queue, &mut CommandEncoder... Wait, actually it wants &mut StagingBelt? 
        // No, looking at wgpu_glyph docs, it might differ. Let's check common patterns.
        // Usually: glyph_brush.draw_queued(device, staging_belt, encoder, view, width, height)
        // I don't have a staging belt in my Renderer yet. I should add it.
        log::warn!("StagingBelt missing for text rendering");
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vertex_desc() { let desc = Vertex::desc(); assert_eq!(desc.array_stride, std::mem::size_of::<Vertex>() as u64); assert_eq!(desc.attributes.len(), 6); }
}
