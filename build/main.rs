mod tiled;

use std::error::Error;
use std::path::PathBuf;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
  let out = PathBuf::from(env::var("OUT_DIR")?);
  let overworld = tiled::get_tile_set_enum("assets/sprites/tiles.tsx", "Tiles")?;

  fs::write(out.join("tiles.rs"), overworld)?;

  Ok(())
}
