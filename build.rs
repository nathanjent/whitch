use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use quote::quote;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable must be specified");

    let playground_filename = "./tilemaps/level1.tmx";
    println!("cargo:rerun-if-changed={playground_filename}");

    let map = tiled::parse_file(Path::new(playground_filename)).unwrap();

    let width = map.width;
    let height = map.height;

    let background_layer = &map.layers[0];
    let background_tiles = extract_tiles(&background_layer.tiles);

    let (bats_x, bats_y) = get_spawn_locations(&map.object_groups[0], "Bat Spawn");

    let mut tile_types = HashMap::new();

    for tile in map.tilesets[0].tiles.iter() {
        if let Some("Collision") = tile.tile_type.as_deref() {
            tile_types.insert(tile.id, 1u8);
        }
    }

    let tile_types =
        (0..map.tilesets[0].tilecount.unwrap()).map(|id| tile_types.get(&(id + 1)).unwrap_or(&0));

    let output = quote! {
        pub const BACKGROUND_MAP: &[u16] = &[#(#background_tiles),*];

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

fn extract_tiles(layer: &'_ tiled::LayerData) -> impl Iterator<Item = u16> + '_ {
    match layer {
        tiled::LayerData::Finite(tiles) => {
            tiles.iter().flat_map(|row| row.iter().map(|tile| tile.gid))
        }
        _ => unimplemented!("cannot use infinite layer"),
    }
    .map(get_map_id)
}

fn get_map_id(tile_id: u32) -> u16 {
    match tile_id {
        0 => 0,
        i => i as u16 - 1,
    }
}

fn get_spawn_locations(
    object_group: &tiled::ObjectGroup,
    enemy_type: &str,
) -> (Vec<u16>, Vec<u16>) {
    let mut spawns = object_group
        .objects
        .iter()
        .filter(|object| object.obj_type == enemy_type)
        .map(|object| (object.x as u16, object.y as u16))
        .collect::<Vec<_>>();

    spawns.sort_by(|a, b| a.0.cmp(&b.0));

    let xs = spawns.iter().map(|pos| pos.0).collect::<Vec<_>>();
    let ys = spawns.iter().map(|pos| pos.1).collect::<Vec<_>>();

    (xs, ys)
}
