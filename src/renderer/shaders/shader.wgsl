struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) rect_size: vec2<f32>,
    @location(4) radius: f32,
    @location(5) mode: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) rect_size: vec2<f32>,
    @location(3) radius: f32,
    @location(4) @interpolate(flat) mode: u32,
    @location(5) local_pos: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    out.color = model.color;
    out.tex_coords = model.tex_coords;
    out.rect_size = model.rect_size;
    out.radius = model.radius;
    out.mode = model.mode;
    
    // Convert normalized device coordinates back to local-ish coordinates for SDF
    out.local_pos = model.tex_coords * model.rect_size;
    
    return out;
}

fn sdf_rounded_rect(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + r;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0))) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (in.mode == 0u) { // Rounded Rect Mode
        let half_size = in.rect_size * 0.5;
        let p = in.local_pos - half_size;
        let d = sdf_rounded_rect(p, half_size, in.radius);
        
        // Anti-aliasing
        let edge_softness = 1.0;
        let alpha = 1.0 - smoothstep(-edge_softness, edge_softness, d);
        
        return vec4<f32>(in.color.rgb, in.color.a * alpha);
    } else if (in.mode == 1u) { // Texture Mode
        return textureSample(t_diffuse, s_diffuse, in.tex_coords) * in.color;
    }
    
    return in.color;
}
