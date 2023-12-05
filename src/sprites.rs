use rand::{thread_rng, Rng};

include!(concat!(env!("OUT_DIR"), "/tiles.rs"));

pub enum Sprite {
  Tiles(Tiles),
}

impl Sprite {
  /// Gets the path to the spritesheet texture.
  pub fn path(&self) -> &'static str {
    match self {
      Self::Tiles(_) => Tiles::path(),
    }
  }
}

impl Tiles {
  pub fn is_grass(&self) -> bool {
    matches!(
      self,
      Self::Grass | Self::Grass1 | Self::Grass2 | Self::Grass3
    )
  }

  pub fn is_sand(&self) -> bool {
    matches!(
      self,
      Self::Beach
        | Self::BeachTop
        | Self::BeachTopLeft
        | Self::BeachTopRight
        | Self::BeachRight
        | Self::BeachLeft
        | Self::BeachBottom
        | Self::BeachBottomLeft
        | Self::BeachBottomRight
    )
  }

  pub fn is_land(&self) -> bool {
    !self.is_water()
  }

  pub fn is_water(&self) -> bool {
    self.is_water_deep() || self.is_water_shallow()
  }

  pub fn is_water_deep(&self) -> bool {
    matches!(self, Self::WaterDeep)
  }

  pub fn is_water_shallow(&self) -> bool {
    matches!(self, Self::WaterShallow)
  }

  pub fn grass<R: Rng>(mut rng: R) -> Tiles {
    match rng.gen::<f32>() {
      v if v < 0.1 => Tiles::Grass1,
      v if v < 0.2 => Tiles::Grass2,
      _ => Tiles::Grass,
    }
  }

  pub fn grass_thread_rng() -> Tiles {
    Self::grass(thread_rng())
  }

  // pub fn water_deep() -> Tiles {
  //   *[Tiles::WaterDeep, Tiles::WaterDeep]
  //     .choose(&mut thread_rng())
  //     .unwrap()
  // }

  // pub fn water_shallow() -> Tiles {
  //   *[
  //     Tiles::WaterShallow,
  //     Tiles::WaterShallowWave,
  //     // Tiles::WaterShallowWave1,
  //   ]
  //   .choose(&mut thread_rng())
  //   .unwrap()
  // }
}
