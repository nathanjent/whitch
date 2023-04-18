#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

use alloc::boxed::Box;
use core::iter::zip;

use agb::{
    display::{
        self,
        object::ObjectController,
        tiled::{InfiniteScrolledMap, TileFormat, TileSet, TileSetting, VRamManager},
    },
    fixnum::{FixedNum, Vector2D},
    include_gfx,
    input::ButtonController,
    interrupt, Gba,
};

include_gfx!("gfx/sprites.toml");
include_gfx!("gfx/tiles.toml");

mod tilemap {
    include!(concat!(env!("OUT_DIR"), "/tilemap.rs"));
}

struct Game<'a> {
    player: Player<'a>,
    input: ButtonController,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum GameState {
    Continue,
    Lost,
    Respawn,
}

struct Player<'a> {
    object_controller: &'a ObjectController,
    position: Vector2D<FixedNum<8>>,
}

impl<'a> Player<'a> {
    fn new(object_controller: &'a ObjectController) -> Self {
        Self {
            object_controller,
            position: (0, 0).into(),
        }
    }
}

impl<'a> Game<'a> {
    fn new(object_controller: &'a ObjectController, respawn: bool) -> Self {
        let mut player = Player::new(&object_controller);
        if respawn {
            player.position = (8, 8).into();
        }

        Self {
            input: ButtonController::new(),
            player,
        }
    }

    fn next(
        &mut self,
        object_controller: &'a ObjectController,
        vram: &mut VRamManager,
    ) -> GameState {
        GameState::Continue
    }
}

pub fn game_with_level(gba: &mut Gba) {
    let vblank = interrupt::VBlank::get();
    vblank.wait_for_vblank();

    let mut respawn = false;

    loop {
        let (bg, mut vram) = gba.display.video.tiled0();
        vram.set_background_palettes(tiles::PALETTES);

        let tileset = TileSet::new(tiles::tiles.tiles, TileFormat::FourBpp);

        let object_controller = gba.display.object.get();

        let mut game = Game::new(&object_controller, respawn);

        let mut background = InfiniteScrolledMap::new(
            bg.background(
                display::Priority::P2,
                display::tiled::RegularBackgroundSize::Background32x32,
            ),
            Box::new(|pos| {
                (
                    &tileset,
                    TileSetting::from_raw(
                        *tilemap::BACKGROUND_MAP
                            .get((pos.x + tilemap::WIDTH * pos.y) as usize)
                            .unwrap_or(&0),
                    ),
                )
            }),
        );

        let bat_spawns = zip(tilemap::BAT_SPAWNS_X.iter(), tilemap::BAT_SPAWNS_Y.iter());

        let start_pos = (8, 8).into();

        let mut between_updates = || {
            vblank.wait_for_vblank();
        };

        background.init(&mut vram, start_pos, &mut between_updates);

        background.commit(&mut vram);
        background.show();

        respawn = loop {
            vblank.wait_for_vblank();
            object_controller.commit();
            match game.next(&object_controller, &mut vram) {
                GameState::Continue => {}
                GameState::Lost => {
                    break false;
                }
                GameState::Respawn => {
                    break true;
                }
            }
        };

        background.clear(&mut vram);
    }
}
