use glyphon::{
    Cache, FontSystem, SwashCache, TextAtlas,
    TextRenderer as GlyphonRenderer, Viewport,
};
use wgpu::{Device, MultisampleState, Queue, RenderPass, TextureFormat};

pub struct TextRenderer {
    pub font_system: FontSystem,
    swash_cache: SwashCache,
    viewport: Viewport,
    atlas: TextAtlas,
    renderer: GlyphonRenderer,
}

impl TextRenderer {
    pub fn new(
        device: &Device,
        queue: &Queue,
        format: TextureFormat,
    ) -> Self {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let viewport = Viewport::new(device, &cache);
        let mut atlas = TextAtlas::new(device, queue, &cache, format);
        let renderer = GlyphonRenderer::new(
            &mut atlas,
            device,
            MultisampleState::default(),
            None,
        );

        Self {
            font_system,
            swash_cache,
            viewport,
            atlas,
            renderer,
        }
    }

    pub fn prepare(
        &mut self,
        device: &Device,
        queue: &Queue,
        sections: Vec<glyphon::TextArea>,
        width: u32,
        height: u32,
    ) {
        self.viewport.update(queue, glyphon::Resolution { width, height });
        
        self.renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                sections,
                &mut self.swash_cache,
            )
            .unwrap();
    }

    pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        self.renderer
            .render(&self.atlas, &self.viewport, render_pass)
            .unwrap();
    }
}
