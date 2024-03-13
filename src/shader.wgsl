// We use separate the x and y instead of using a vec2 to avoid wgsl padding.
struct AppState {
	time: i32,
}

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@group(0)
@binding(0)
var<uniform> app_state: AppState;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

		let xcoords = vec3<f32>(-0.5, 0.5, 0.0);
		let ycoords = vec3<f32>(-0.5, -0.5, 0.5);

    out.position = vec4<f32>(xcoords[in.vertex_index], ycoords[in.vertex_index], 1.0, 1.0);

		if (in.vertex_index == 0u) {
    	out.color = vec4<f32>(sin(0.01 * f32(app_state.time)) + 1, 0.0, 0.0, 1.0);
    	// out.color = vec4<f32>(1.0, 0.0, 0.0, 1.0);
		} else if (in.vertex_index == 1u) {
			out.color = vec4<f32>(0.0, sin(0.01 * f32(app_state.time)) + 1, 0.0, 1.0);
			// out.color = vec4<f32>(0.0, 1.0, 0.0, 1.0);
		} else {
			out.color = vec4<f32>(0.0, 0.0, sin(0.01 * f32(app_state.time)) + 1, 1.0);
			// out.color = vec4<f32>(0.0, 0.0, 1.0, 1.0);
		}

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	return in.color;
}
