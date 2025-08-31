@group(0) @binding(0) var<uniform> color: vec4<f32>;
@group(0) @binding(1) var<uniform> uv_rect: vec4<f32>; // u0,v0,u1,v1
@group(0) @binding(2) var<uniform> neighbor_mask: u32;
@group(0) @binding(3) var<uniform> border_color: vec4<f32>;

@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var samp: sampler;

struct VertexOut { @builtin(position) pos: vec4<f32>, @location(0) uv: vec2<f32> };

fn in_mask(mask: u32, bit: u32) -> bool { return (mask & (1u << bit)) != 0u; }

@fragment
fn fragment(in: VertexOut) -> @location(0) vec4<f32> {
  let base_uv = uv_rect.xy + in.uv * (uv_rect.zw - uv_rect.xy);
  let texel = textureSample(tex, samp, base_uv);

  let uv = in.uv;
  let p = uv * 2.0 - vec2<f32>(1.0, 1.0);
  let k = vec3<f32>(0.8660254, 0.5, 0.57735);
  let pabs = vec2<f32>(abs(p.x), abs(p.y));
  let d = max(dot(pabs, k.xy), pabs.x) - k.z;
  let alpha = clamp(1.0 - smoothstep(0.0, 0.02, d), 0.0, 1.0);

  var edge = 0.0;
  let t = smoothstep(0.015, 0.03, abs(d));
  if (!in_mask(neighbor_mask, 0u)) { edge = max(edge, t); }
  if (!in_mask(neighbor_mask, 1u)) { edge = max(edge, t); }
  if (!in_mask(neighbor_mask, 2u)) { edge = max(edge, t); }
  if (!in_mask(neighbor_mask, 3u)) { edge = max(edge, t); }
  if (!in_mask(neighbor_mask, 4u)) { edge = max(edge, t); }
  if (!in_mask(neighbor_mask, 5u)) { edge = max(edge, t); }

  let base = vec4<f32>(texel.rgb * color.rgb, texel.a * alpha * color.a);
  let border = vec4<f32>(border_color.rgb, border_color.a * edge * alpha);
  return base + border * (1.0 - base.a);
}
