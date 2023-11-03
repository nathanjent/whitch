use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

use quote::quote;
use tiled::Loader;
use tiled::Layer;

fn main() {
    let mut loader = Loader::new();

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable must be specified");

    let level1_filename = "./tilemaps/level1.tmx";
    println!("cargo:rerun-if-changed={level1_filename}");

    let map = loader.load_tmx_map(level1_filename).unwrap();

    let width = map.width;
    let height = map.height;

    let background_tiles= map.tilesets()[0].tiles().map(|t| t.0);
    let ground_tiles= map.tilesets()[1].tiles().map(|t| t.0);

    let (bats_x, bats_y) = get_spawn_locations(&map.get_layer(2).unwrap(), "Bat Spawn");

    let mut tile_types = HashMap::new();

    for tile in map.tilesets()[0].tiles() {
        if let Some("Collision") = tile.1.user_type.as_deref() {
            tile_types.insert(tile.0, 1u8);
        }
    }

    let tile_types =
        (0..map.tilesets()[0].tilecount).map(|id| tile_types.get(&(id + 1)).unwrap_or(&0));

    let output = quote! {
        pub const BACKGROUND_MAP: &[u16] = &[#(#background_tiles),*];
        pub const GROUND_MAP: &[u16] = &[#(#ground_tiles),*];

        pub const WIDTH: i32 = #width as i32;
        pub const HEIGHT: i32 = #height as i32;

        pub const BAT_SPAWNS_X: &[u16] = &[#(#bats_x),*];
        pub const BAT_SPAWNS_Y: &[u16] = &[#(#bats_y),*];

        pub const TILE_TYPES: &[u8] = &[#(#tile_types),*];
    };

    let output_file = File::create(format!("{out_dir}/tilemap.rs"))
        .expect("failed to open tilemap.rs file for writing");
    let mut writer = BufWriter::new(output_file);

    write!(&mut writer, "{output}").unwrap();
}

fn get_spawn_locations(
    object_group: &Layer,
    enemy_type: &str,
) -> (Vec<u16>, Vec<u16>) {
    let mut spawns = object_group
        .as_object_layer().unwrap()
        .objects()
        .filter(|object| object.user_type == enemy_type)
        .map(|object| (object.x as u16, object.y as u16))
        .collect::<Vec<_>>();

    spawns.sort_by(|a, b| a.0.cmp(&b.0));

    let xs = spawns.iter().map(|pos| pos.0).collect::<Vec<_>>();
    let ys = spawns.iter().map(|pos| pos.1).collect::<Vec<_>>();

    (xs, ys)
}
