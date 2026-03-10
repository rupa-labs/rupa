use glyphon::{
    FontSystem, SwashCache, TextAtlas, TextRenderer as GlyphonTextRenderer,
    TextArea, Resolution
};
use wgpu::{Device, Queue, TextureFormat, RenderPass, MultisampleState};

pub struct TextRenderer {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub atlas: TextAtlas,
    pub renderer: GlyphonTextRenderer,
}

impl TextRenderer {
    pub fn new(device: &Device, queue: &Queue, format: TextureFormat) -> Self {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let mut atlas = TextAtlas::new(device, queue, format);
        let renderer = GlyphonTextRenderer::new(&mut atlas, device, MultisampleState::default(), None);

        Self {
            font_system,
            swash_cache,
            atlas,
            renderer,
        }
    }

    pub fn prepare(
        &mut self,
        device: &Device,
        queue: &Queue,
        text_areas: Vec<TextArea>,
        width: u32,
        height: u32,
    ) {
        self.renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.atlas,
                Resolution { width, height },
                text_areas,
                &mut self.swash_cache,
            )
            .unwrap();
    }

    pub fn render<'a>(&'a self, pass: &mut RenderPass<'a>) {
        self.renderer.render(&self.atlas, pass).unwrap();
    }
}
