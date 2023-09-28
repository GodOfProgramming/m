#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_view_bindings globals
#import m::common as common

struct Info {
  resolution: vec2<f32>,
}

@group(1) @binding(0)
var<uniform> info: Info;

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
  var p: vec2<f32> = -0.5 + in.position.xy / info.resolution.xy;
  p.x *= info.resolution.x / info.resolution.y;

  var color: f32 = 3.0 - (3.0 * length(2.0 * p));

  let coord = vec3<f32>(atan(p.y / p.x) / 6.2832 + 0.5, length(p) * 0.4, 0.5);

  for (var i: i32 = 1; i <= 7; i += 1) {
    let power = pow(2.0, f32(i));
    color += (1.5 / power) * common::snoise(coord + vec3<f32>(0.0, -globals.time * 0.05, globals.time * 0.01), power * 16.0);
  }

  return vec4<f32>(color, pow(max(color, 0.0), 2.0) * 0.4, pow(max(color, 0.0), 3.0) * 0.15, 1.0) * vec4<f32>(abs(sin(globals.time)), abs(cos(globals.time)), abs(tan(globals.time)), 1.0);
}