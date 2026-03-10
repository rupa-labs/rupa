use std::sync::Arc;
use wgpu::*;
use wgpu::util::StagingBelt;
use rupa_core::Vec2;
use rupa_vnode::TextAlign;
use crate::renderer::{Renderer as BaseRenderer, RenderCore};
use super::batcher::{Batcher, Vertex};
use super::texture::Texture;
use super::text_renderer::TextRenderer;

struct StoredText {
    buffer: glyphon::Buffer,
    pos: Vec2,
    color: [f32; 4],
}

pub struct Renderer {
    pub core: RenderCore,
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    
    pub render_pipeline: RenderPipeline,
    pub batcher: Batcher,
    pub default_texture: Texture,
    pub text_renderer: TextRenderer,
    pub staging_belt: StagingBelt,
    
    text_entries: Vec<StoredText>,
    current_encoder: Option<CommandEncoder>,
    current_view: Option<TextureView>,
    current_output: Option<SurfaceTexture>,
}

impl Renderer {
    pub async fn new<W>(window: Arc<W>, width: u32, height: u32, scale_factor: f32) -> Self 
    where
        W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle + Send + Sync + 'static
    {
        let instance = Instance::new(InstanceDescriptor { backends: Backends::all(), ..Default::default() });
        let surface = instance.create_surface(window).expect("Failed to create surface");
        let adapter = instance.request_adapter(&RequestAdapterOptions { power_preference: PowerPreference::HighPerformance, compatible_surface: Some(&surface), force_fallback_adapter: false }).await.unwrap();
        let (device, queue) = adapter.request_device(&DeviceDescriptor { label: Some("Rupa Device"), ..Default::default() }, None).await.unwrap();
        
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter().copied().find(|f| f.is_srgb()).unwrap_or(surface_caps.formats[0]);
        let config = SurfaceConfiguration { usage: TextureUsages::RENDER_ATTACHMENT, format: surface_format, width, height, present_mode: surface_caps.present_modes[0], alpha_mode: surface_caps.alpha_modes[0], view_formats: vec![], desired_maximum_frame_latency: 2 };
        surface.configure(&device, &config);

        let texture_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor { 
            entries: &[
                BindGroupLayoutEntry { binding: 0, visibility: ShaderStages::FRAGMENT, ty: BindingType::Texture { multisampled: false, view_dimension: TextureViewDimension::D2, sample_type: TextureSampleType::Float { filterable: true } }, count: None },
                BindGroupLayoutEntry { binding: 1, visibility: ShaderStages::FRAGMENT, ty: BindingType::Sampler(SamplerBindingType::Filtering), count: None }
            ], 
            label: Some("texture_bind_group_layout") 
        });

        let default_img = image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(1, 1, image::Rgba([255, 255, 255, 255])));
        let default_texture = Texture::from_image(&device, &queue, &default_img, Some("Default White"), &texture_bind_group_layout);
        
        let shader = device.create_shader_module(include_wgsl!("shaders/shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor { label: Some("Render Pipeline Layout"), bind_group_layouts: &[&texture_bind_group_layout], push_constant_ranges: &[] });
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Rupa Render Pipeline"), layout: Some(&render_pipeline_layout),
            vertex: VertexState { module: &shader, entry_point: "vs_main", buffers: &[Vertex::desc()] },
            fragment: Some(FragmentState { module: &shader, entry_point: "fs_main", targets: &[Some(ColorTargetState { format: config.format, blend: Some(BlendState::ALPHA_BLENDING), write_mask: ColorWrites::ALL })] }),
            primitive: PrimitiveState { topology: PrimitiveTopology::TriangleList, ..Default::default() },
            depth_stencil: None, multisample: MultisampleState::default(), multiview: None,
        });

        let batcher = Batcher::new(&device, 10000);
        let text_renderer = TextRenderer::new(&device, &queue, config.format);
        let staging_belt = StagingBelt::new(1024);

        Self { 
            core: RenderCore::new(width as f32, height as f32, scale_factor),
            surface, device, queue, config, 
            render_pipeline, batcher, default_texture, text_renderer, staging_belt,
            text_entries: Vec::new(),
            current_encoder: None, current_view: None, current_output: None,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32, scale_factor: f32) { 
        if width > 0 && height > 0 { 
            self.core.logical_size = Vec2::new(width as f32, height as f32);
            self.core.scale_factor = scale_factor;
            self.config.width = width; 
            self.config.height = height; 
            self.surface.configure(&self.device, &self.config); 
        } 
    }

    pub fn begin_frame(&mut self) -> Result<(), SurfaceError> { 
        let output = self.surface.get_current_texture()?; 
        let view = output.texture.create_view(&TextureViewDescriptor::default()); 
        let encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: Some("Rupa Frame Encoder") }); 
        
        self.text_entries.clear();
        self.current_output = Some(output);
        self.current_view = Some(view);
        self.current_encoder = Some(encoder);
        Ok(()) 
    }
}

impl crate::renderer::TextMeasurer for Renderer {
    fn measure(&self, text: &str, size: f32) -> Vec2 {
        let mut font_system = glyphon::FontSystem::new(); 
        let tsize = size * self.core.scale_factor;
        let mut buffer = glyphon::Buffer::new(&mut font_system, glyphon::Metrics::new(tsize, tsize));
        buffer.set_text(&mut font_system, text, glyphon::Attrs::new().family(glyphon::Family::SansSerif), glyphon::Shaping::Advanced);
        buffer.shape_until_scroll(&mut font_system);
        
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;
        for run in buffer.layout_runs() {
            width = width.max(run.line_w);
            height += tsize; // Use metrics line height as fallback
        }
        Vec2::new(width / self.core.scale_factor, height / self.core.scale_factor)
    }
}

impl BaseRenderer for Renderer {
    fn core(&self) -> &RenderCore { &self.core }
    fn core_mut(&mut self) -> &mut RenderCore { &mut self.core }

    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], radius: f32) {
        let scale = self.core.scale_factor;
        let tx = (x + self.core.camera_offset.x) * self.core.camera_zoom * scale; 
        let ty = (y + self.core.camera_offset.y) * self.core.camera_zoom * scale;
        let tw = w * self.core.camera_zoom * scale; 
        let th = h * self.core.camera_zoom * scale;
        
        let x_norm = (tx / self.core.logical_size.x) * 2.0 - 1.0; 
        let y_norm = 1.0 - (ty / self.core.logical_size.y) * 2.0;
        let w_norm = (tw / self.core.logical_size.x) * 2.0; 
        let h_norm = (th / self.core.logical_size.y) * 2.0;
        
        let size = [tw, th];
        let radius = radius * self.core.camera_zoom * scale;

        self.batcher.add_rect([
            Vertex { position: [x_norm, y_norm], color, tex_coords: [0.0, 0.0], rect_size: size, radius, mode: 0 },
            Vertex { position: [x_norm + w_norm, y_norm], color, tex_coords: [1.0, 0.0], rect_size: size, radius, mode: 0 },
            Vertex { position: [x_norm + w_norm, y_norm - h_norm], color, tex_coords: [1.0, 1.0], rect_size: size, radius, mode: 0 },
            Vertex { position: [x_norm, y_norm - h_norm], color, tex_coords: [0.0, 1.0], rect_size: size, radius, mode: 0 },
        ]);
    }

    fn draw_text(&mut self, content: &str, x: f32, y: f32, width: f32, size: f32, color: [f32; 4], _align: TextAlign) {
        let scale = self.core.scale_factor;
        let tx = (x + self.core.camera_offset.x) * self.core.camera_zoom * scale; 
        let ty = (y + self.core.camera_offset.y) * self.core.camera_zoom * scale;
        let tsize = size * self.core.camera_zoom * scale;
        let twidth = width * self.core.camera_zoom * scale;

        let mut buffer = glyphon::Buffer::new(&mut self.text_renderer.font_system, glyphon::Metrics::new(tsize, tsize));
        buffer.set_size(&mut self.text_renderer.font_system, twidth, self.core.logical_size.y * scale);
        buffer.set_text(&mut self.text_renderer.font_system, content, glyphon::Attrs::new().family(glyphon::Family::SansSerif), glyphon::Shaping::Advanced);
        buffer.shape_until_scroll(&mut self.text_renderer.font_system);

        self.text_entries.push(StoredText { buffer, pos: Vec2::new(tx, ty), color });
    }

    fn draw_outline(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        let scale = self.core.scale_factor;
        let tx = (x + self.core.camera_offset.x) * self.core.camera_zoom * scale; 
        let ty = (y + self.core.camera_offset.y) * self.core.camera_zoom * scale;
        let tw = w * self.core.camera_zoom * scale; 
        let th = h * self.core.camera_zoom * scale;
        
        let x_norm = (tx / self.core.logical_size.x) * 2.0 - 1.0; 
        let y_norm = 1.0 - (ty / self.core.logical_size.y) * 2.0;
        let w_norm = (tw / self.core.logical_size.x) * 2.0; 
        let h_norm = (th / self.core.logical_size.y) * 2.0;
        
        let size = [tw, th];
        self.batcher.add_rect([
            Vertex { position: [x_norm, y_norm], color, tex_coords: [0.0, 0.0], rect_size: size, radius: 0.0, mode: 1 },
            Vertex { position: [x_norm + w_norm, y_norm], color, tex_coords: [1.0, 0.0], rect_size: size, radius: 0.0, mode: 1 },
            Vertex { position: [x_norm + w_norm, y_norm - h_norm], color, tex_coords: [1.0, 1.0], rect_size: size, radius: 0.0, mode: 1 },
            Vertex { position: [x_norm, y_norm - h_norm], color, tex_coords: [0.0, 1.0], rect_size: size, radius: 0.0, mode: 1 },
        ]);
    }

    fn push_clip(&mut self, _x: f32, _y: f32, _w: f32, _h: f32) {}
    fn pop_clip(&mut self) {}

    fn present(&mut self) {
        let mut encoder = self.current_encoder.take().expect("present() called before begin_frame()");
        let view = self.current_view.take().unwrap();
        let output = self.current_output.take().unwrap();

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Rupa Main Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view, resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None, 
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.render_pipeline); 
            pass.set_bind_group(0, &self.default_texture.bind_group, &[]); 
            self.batcher.flush(&self.queue, &mut pass);
        }

        let text_areas: Vec<glyphon::TextArea> = self.text_entries.iter().map(|t| {
            glyphon::TextArea {
                buffer: &t.buffer, left: t.pos.x, top: t.pos.y, scale: 1.0,
                bounds: glyphon::TextBounds { left: 0, top: 0, right: self.core.logical_size.x as i32, bottom: self.core.logical_size.y as i32 },
                default_color: glyphon::Color::rgba((t.color[0] * 255.0) as u8, (t.color[1] * 255.0) as u8, (t.color[2] * 255.0) as u8, (t.color[3] * 255.0) as u8),
            }
        }).collect();

        self.text_renderer.prepare(&self.device, &self.queue, text_areas, self.core.logical_size.x as u32, self.core.logical_size.y as u32);
        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Text Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view, resolve_target: None,
                    ops: Operations { load: LoadOp::Load, store: StoreOp::Store },
                })],
                depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None,
            });
            self.text_renderer.render(&mut pass);
        }

        self.staging_belt.finish();
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        self.staging_belt.recall();
    }
}
