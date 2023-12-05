#import bevy_pbr::forward_io::VertexOutput;
// we can import items from shader modules in the assets folder with a quoted path
// #import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

// @group(1) @binding(0) var<uniform> material: ChunkMaterial;
@group(1) @binding(1) var base_color_texture: texture_2d<f32>;
@group(1) @binding(2) var base_color_sampler: sampler;
@group(1) @binding(3) var<storage> tiles: array<u32>;
@group(1) @binding(4) var<uniform> tile_size: u32;

const TILE_OFFSET = 0.0;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
  var d = textureDimensions(base_color_texture);
  var len = arrayLength(&tiles);
  var dim = sqrt(f32(len));

  var idx = coords_to_tile_index(mesh.uv, dim);
  var min = index_to_texture_coords(idx);
  var max = min + vec2(f32(tile_size) - TILE_OFFSET);

  var px = (mesh.uv.x % (1.0 / dim)) * dim;
  var py = (mesh.uv.y % (1.0 / dim)) * dim;

  var x = mix(f32(min.x), f32(max.x), px) / f32(d.x);
  var y = mix(f32(min.y), f32(max.y), py) / f32(d.y);
  
  return textureSample(base_color_texture, base_color_sampler, vec2(x, y));
}

/// Gets the [0..1] texture coords for the given tile index
fn index_to_texture_coords(i: u32) -> vec2<f32> {
  var d = textureDimensions(base_color_texture);
  var z = i * tile_size;
  var x = clamp(z % d.x            , 0u, d.x - tile_size);
  var y = clamp(z / d.x * tile_size, 0u, d.y - tile_size);

  return vec2(f32(x) + TILE_OFFSET, f32(y) + TILE_OFFSET);
}

/// Gets the index in the tiles array for the given mesh uv.
fn coords_to_tile_index(uv: vec2<f32>, fdim: f32) -> u32 {
  var dim = u32(fdim);
  var px = u32(floor(uv.x * fdim));
  var py = u32(floor((1.0 - uv.y) * fdim));

  return tiles[px * dim + py];
}
