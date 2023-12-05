use heck::ToUpperCamelCase;
use quote::{format_ident, quote};
use std::path::Path;
use tiled::Loader;

pub fn get_tile_set_enum<P: AsRef<Path>>(path: P, name: &str) -> anyhow::Result<String> {
  let mut loader = Loader::new();

  let name = format_ident!("{name}");

  let tileset = loader.load_tsx_tileset(&path)?;
  let tiles = tileset
    .tiles()
    .filter_map(|(i, tile)| tile.user_type.as_ref().map(|val| (i, val.clone())))
    .collect::<Vec<_>>();

  let path = tileset.image.unwrap();
  let path = path.source.to_string_lossy().replace("assets/", "");

  let variants = tiles.iter().map(|(i, name)| {
    let i = *i as isize;
    let ident = name.to_upper_camel_case();
    let ident = format_ident!("{ident}");
    let docs = format!(" Tile type `{name}` at index `{i}`");
    let is_default = if i == 0 {
      Some(quote! { #[default] })
    } else {
      None
    };

    quote! {
      #[doc = #docs]
      #is_default
      #ident = #i
    }
  });

  let count = tiles.len();
  let names = tiles.iter().map(|(_, name)| {
    let name = name.to_upper_camel_case();

    format_ident!("{name}")
  });

  Ok(prettyplease::unparse(&syn::parse2(quote! {
    #[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
    pub enum #name {
      #(#variants),*
    }

    impl #name {
      /// Gets all sprites.
      pub const ALL: [#name; #count] = [
        #(#name::#names),*
      ];

      /// Gets the path to the tile set image.
      pub fn path() -> &'static str {
        #path
      }
    }
  })?))
}
