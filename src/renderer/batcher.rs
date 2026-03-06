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
    pub mode: u32 
}

impl Vertex {
    const ATTRIBS: [VertexAttribute; 6] = vertex_attr_array![0 => Float32x2, 1 => Float32x4, 2 => Float32x2, 3 => Float32x2, 4 => Float32, 5 => Uint32];
    pub fn desc() -> VertexBufferLayout<'static> { 
        VertexBufferLayout { 
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress, 
            step_mode: VertexStepMode::Vertex, 
            attributes: &Self::ATTRIBS 
        } 
    }
}

pub struct Batcher {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub max_batch_size: usize,
}

impl Batcher {
    pub fn new(device: &Device, max_batch_size: usize) -> Self {
        let vertex_buffer = device.create_buffer(&BufferDescriptor { 
            label: Some("Vertex Buffer"), 
            size: (std::mem::size_of::<Vertex>() * max_batch_size * 4) as BufferAddress, 
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST, 
            mapped_at_creation: false 
        });
        let index_buffer = device.create_buffer(&BufferDescriptor { 
            label: Some("Index Buffer"), 
            size: (std::mem::size_of::<u32>() * max_batch_size * 6) as BufferAddress, 
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST, 
            mapped_at_creation: false 
        });

        Self {
            vertices: Vec::with_capacity(max_batch_size * 4),
            indices: Vec::with_capacity(max_batch_size * 6),
            vertex_buffer,
            index_buffer,
            max_batch_size,
        }
    }

    pub fn add_rect(&mut self, vertices: [Vertex; 4]) {
        if self.vertices.len() + 4 > self.max_batch_size * 4 {
            return; // TODO: Flush automatically or return error
        }
        let base_idx = self.vertices.len() as u32;
        self.vertices.extend_from_slice(&vertices);
        self.indices.extend_from_slice(&[
            base_idx, base_idx + 1, base_idx + 2, 
            base_idx, base_idx + 2, base_idx + 3
        ]);
    }

    pub fn flush<'a>(&'a mut self, queue: &Queue, render_pass: &mut RenderPass<'a>) {
        if self.indices.is_empty() {
            return;
        }
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&self.indices));
        
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
        
        self.vertices.clear();
        self.indices.clear();
    }
}
