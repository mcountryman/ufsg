use super::chunk::{ChunkId, ChunkTiles};
use super::CHUNK_TILES_SQ;
use crate::sprites::Tiles;
use bevy::math::DVec2;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

/// Tile map generation configuration.
#[derive(Resource)]
pub struct GenerateConf {
  /// The map seed.
  pub seed: u32,
  /// The continent scale defined as a value [0-1].
  pub continent_scale: f64,
}

pub fn generate(conf: &GenerateConf, chunk_id: ChunkId, chunk_tiles: &mut ChunkTiles) {
  let dim = CHUNK_TILES_SQ as f64;
  let pos = chunk_id.as_dvec2();

  let mut rng = xor_shift_rng(conf, chunk_id.x, chunk_id.y);
  let continents = continents_noise(conf);

  for x in 0..CHUNK_TILES_SQ as u32 {
    for y in 0..CHUNK_TILES_SQ as u32 {
      let px = pos.x * dim + x as f64;
      let py = pos.y * dim + y as f64;
      let tile = match continents.get([px, py]) {
        v if v <= 0.1 => Tiles::WaterDeep,
        v if v <= 0.2 => Tiles::WaterShallow,
        v if v <= 0.25 => Tiles::Beach,
        _ => Tiles::grass(&mut rng),
      };

      chunk_tiles.set(UVec2::new(x, y), tile);
    }
  }
}

fn xor_shift_rng(conf: &GenerateConf, x: i32, y: i32) -> XorShiftRng {
  let mut seed = [0u8; 16];

  let x = (x.abs() * 100) as u32;
  let y = (y.abs() * 100) as u32;
  let z = conf.seed;
  let w = x.wrapping_mul(2).wrapping_add(y.wrapping_div(2));

  seed[0..4].copy_from_slice(x.to_le_bytes().as_slice());
  seed[4..8].copy_from_slice(y.to_le_bytes().as_slice());
  seed[8..12].copy_from_slice(z.to_le_bytes().as_slice());
  seed[12..16].copy_from_slice(w.to_le_bytes().as_slice());

  let mut rng = XorShiftRng::from_seed(seed);

  for _ in 0..32 {
    rng.gen::<u64>();
  }

  rng
}

fn continents_noise(conf: &GenerateConf) -> impl NoiseFn<f64, 2> {
  let continent_scale = conf.continent_scale.clamp(0.0, 1.0);

  let freq_min = 1.0;
  let freq_max = 4.0;
  let freq = freq_min + (freq_max - freq_min) * continent_scale;

  let scale_min = 0.005;
  let scale_max = 0.0005;
  let scale = scale_min + (scale_max - scale_min) * continent_scale;

  let continents = FbmOne {
    source: Perlin::new(conf.seed),
    octaves: 6,
    frequency: freq,
    lacunarity: 2.25,
    persistence: 0.5,
  };

  let continents = noise::Clamp::new(continents)
    .set_lower_bound(0.0)
    .set_upper_bound(1.0);

  noise::ScalePoint::new(continents)
    .set_x_scale(scale)
    .set_y_scale(scale)
}

struct FbmOne<T> {
  source: T,
  octaves: usize,
  frequency: f64,
  persistence: f64,
  lacunarity: f64,
}

impl<T> NoiseFn<f64, 2> for FbmOne<T>
where
  T: NoiseFn<f64, 2>,
{
  fn get(&self, point: [f64; 2]) -> f64 {
    let mut point = DVec2::new(point[0], point[1]);
    let mut result = 0.0;

    point *= self.frequency;

    for x in 0..self.octaves {
      // Get the signal.
      let mut signal = self.source.get(*point.as_ref());

      // Scale the amplitude appropriately for this frequency.
      signal *= self.persistence.powi(x as i32);

      // Add the signal to the result.
      result += signal;

      // Increase the frequency for the next octave.
      point *= self.lacunarity;
    }

    // Scale the result into the [-1,1] range
    result
  }
}

#[cfg(test)]
mod tests {
  use super::GenerateConf;
  use crate::sprites::Tiles;
  use crate::tilemap::chunk::ChunkTiles;
  use crate::tilemap::CHUNK_TILES_SQ;
  use bevy::math::{IVec2, UVec2};
  use image::{Rgb, RgbImage};

  #[test]
  #[ignore]
  fn generate_30x30_image() {
    let dim = 30u32;
    let size = dim * CHUNK_TILES_SQ as u32;
    let conf = GenerateConf {
      seed: 0xdead,
      continent_scale: 1.0,
    };

    let mut image = RgbImage::new(size, size);

    for cx in 0..dim {
      for cy in 0..dim {
        let mut tiles = ChunkTiles::default();

        super::generate(&conf, IVec2::new(cx as _, cy as _).into(), &mut tiles);

        for x in 0..CHUNK_TILES_SQ as _ {
          for y in 0..CHUNK_TILES_SQ as _ {
            let tile = UVec2::new(x, y);
            let tile = match tiles.get(tile).unwrap() {
              Tiles::Grass => Rgb([0, 255, 0]),
              Tiles::WaterDeep => Rgb([0, 0, 255]),
              Tiles::WaterShallow => Rgb([100, 100, 255]),

              Tiles::Void => Rgb([255, 0, 0]),

              tile => {
                println!("{tile:?}");
                Rgb([0, 0, 0])
              }
            };

            image.put_pixel(
              cx * CHUNK_TILES_SQ as u32 + x,
              cy * CHUNK_TILES_SQ as u32 + y,
              tile,
            );
          }
        }
      }
    }

    image.save("target/test.png").unwrap();
  }
}
