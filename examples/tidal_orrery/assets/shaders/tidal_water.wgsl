#import bevy_pbr::{
    mesh_view_bindings::{globals, view},
    forward_io::VertexOutput,
}

@group(3) @binding(0) var<uniform> deep_color: vec4<f32>;
@group(3) @binding(1) var<uniform> crest_color: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let p = in.world_position.xz;
    let time = globals.time * 0.24;
    let broad_wave = sin(p.x * 0.38 + p.y * 0.57 - time);
    let cross_wave = sin(p.x * 0.61 - p.y * 0.29 + time * 0.72);
    let flow = p.y * 3.3 + broad_wave * 0.7 + cross_wave * 0.42 - time * 2.0;
    let ripple = abs(sin(flow));
    let line_width = max(fwidth(ripple), 0.012);
    let fine_ripple = 1.0 - smoothstep(0.045, 0.045 + line_width * 1.5, ripple);

    let crossing = sin(p.x * 1.7 + p.y * 1.1 + broad_wave - time * 0.6);
    let caustic = pow(clamp(1.0 - abs(crossing), 0.0, 1.0), 14.0);
    let broken_crest = smoothstep(0.08, 0.85, cross_wave * 0.5 + 0.5);
    let radius = length(p);
    let shallows = smoothstep(15.0, 24.0, radius);
    let top_surface = smoothstep(0.25, 0.85, in.world_normal.y);

    let view_direction = normalize(view.world_position.xyz - in.world_position.xyz);
    let grazing = pow(clamp(1.0 - dot(view_direction, normalize(in.world_normal)), 0.0, 1.0), 3.0);
    let moon_path = exp(-pow((p.x + broad_wave * 0.22) / 2.8, 2.0));
    let moon_reach = smoothstep(-4.0, 1.0, p.y) * (1.0 - smoothstep(14.0, 22.0, p.y));
    let glint = fine_ripple * broken_crest * moon_path * moon_reach;

    var color = deep_color.rgb * (0.83 + broad_wave * 0.055 + cross_wave * 0.035);
    color += crest_color.rgb * top_surface * (
        shallows * 0.075
        + caustic * 0.028
        + fine_ripple * broken_crest * 0.075
        + grazing * 0.055
        + glint * 0.48
    );
    return vec4<f32>(color, 1.0);
}
