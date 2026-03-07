use wgpu::*;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
    pub rect_size: [f32; 2],
    pub radius: f32,
    pub mode: u32,
}

impl Vertex {
    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute { offset: 0, shader_location: 0, format: VertexFormat::Float32x2 },
                VertexAttribute { offset: 8, shader_location: 1, format: VertexFormat::Float32x4 },
                VertexAttribute { offset: 24, shader_location: 2, format: VertexFormat::Float32x2 },
                VertexAttribute { offset: 32, shader_location: 3, format: VertexFormat::Float32x2 },
                VertexAttribute { offset: 40, shader_location: 4, format: VertexFormat::Float32 },
                VertexAttribute { offset: 44, shader_location: 5, format: VertexFormat::Uint32 },
            ],
        }
    }
}

pub struct Batcher {
    pub vertices: Vec<Vertex>,
    pub vertex_buffer: Buffer,
    pub capacity: usize,
}

impl Batcher {
    pub fn new(device: &Device, capacity: usize) -> Self {
        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: (capacity * std::mem::size_of::<Vertex>()) as BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            vertices: Vec::with_capacity(capacity),
            vertex_buffer,
            capacity,
        }
    }

    pub fn add_rect(&mut self, rect_vertices: [Vertex; 4]) {
        if self.vertices.len() + 6 <= self.capacity {
            self.vertices.push(rect_vertices[0]);
            self.vertices.push(rect_vertices[1]);
            self.vertices.push(rect_vertices[2]);
            self.vertices.push(rect_vertices[0]);
            self.vertices.push(rect_vertices[2]);
            self.vertices.push(rect_vertices[3]);
        }
    }

    pub fn flush<'a>(& 'a mut self, queue: &Queue, pass: &mut RenderPass<'a>) {
        if !self.vertices.is_empty() {
            queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
            // Ensure vertex buffer slice is valid for the duration of the pass
            // We use a broader lifetime or just don't deref if not needed
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.draw(0..self.vertices.len() as u32, 0..1);
            self.vertices.clear();
        }
    }
}
