#import bevy_pbr::mesh_view_bindings::{globals, view}
#import bevy_pbr::forward_io::VertexOutput

@group(3) @binding(0) var<uniform> material: CrystalMaterial;

struct CrystalMaterial {
    base_color: vec4<f32>,
    emissive: vec4<f32>,
}

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time;
    let pulse = (sin(time * 2.0) + 1.0) * 0.5;

    // Simple fresnel effect
    let view_dir = normalize(view.world_position.xyz - in.world_position.xyz);
    let normal = normalize(in.world_normal);
    let fresnel = pow(1.0 - max(dot(view_dir, normal), 0.0), 3.0);

    let color = material.base_color.rgb + material.emissive.rgb * (pulse * 0.5 + 0.5) + vec3<f32>(fresnel);

    return vec4<f32>(color, material.base_color.a);
}
