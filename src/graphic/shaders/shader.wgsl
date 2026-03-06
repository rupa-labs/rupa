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
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    out.color = model.color;
    out.tex_coords = model.tex_coords;
    out.rect_size = model.rect_size;
    out.radius = model.radius;
    out.mode = model.mode;
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

// SDF function for a rounded rectangle
fn sd_rounded_box(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + r;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0))) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 1. Calculate the Signed Distance to the rounded edge
    // Map tex_coords (0 to 1) to local coordinate system (-size/2 to size/2)
    let local_p = (in.tex_coords - 0.5) * in.rect_size;
    let d = sd_rounded_box(local_p, in.rect_size * 0.5, in.radius);

    // 2. Antialiasing (smoothstep)
    let alpha = 1.0 - smoothstep(-1.0, 0.0, d);
    
    // 3. Final Color calculation
    var final_color: vec4<f32>;
    if (in.mode == 1u) {
        final_color = textureSample(t_diffuse, s_diffuse, in.tex_coords) * in.color;
    } else {
        final_color = in.color;
    }

    return vec4<f32>(final_color.rgb, final_color.a * alpha);
}
