#define_import_path m::common

fn snoise(uv_in: vec3<f32>, res: f32) -> f32 {
  let s = vec3<f32>(1.0e0, 1.0e2, 1.0e3);

  let uv = uv_in * res;

  let uv0 = floor(uv % res) * s;
  let uv1 = floor(uv + vec3<f32>(1.0));

  var f: vec3<f32> = fract(uv); f = f * f * (3.0 - 2.0 * f);

  let v = vec4<f32>(
    uv0.x + uv0.y + uv0.z,
    uv1.x + uv0.y + uv0.z,
    uv0.x + uv1.y + uv0.z,
    uv1.x + uv1.y + uv0.z
  );

  var r: vec4<f32> = fract(sin(v * 1.0e-1) * 1.0e3);
  let r0 = mix(mix(r.x, r.y, f.x), mix(r.z, r.w, f.x), f.y);

  r = fract(sin((v + uv1.z - uv0.z)*1.0e-1) * 1.0e3);
  let r1 = mix(mix(r.x, r.y, f.x), mix(r.z, r.w, f.x), f.y);

  return mix(r0, r1, f.z) * 2.0 - 1.0;
}
