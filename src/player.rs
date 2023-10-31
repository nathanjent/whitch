use agb::display::object::Object;
use agb::input::{self, Button, ButtonController};
use agb::{
    display::object::{Graphics, OamManaged, Tag, TagMap},
    fixnum::Vector2D,
    include_aseprite,
};

const GRAPHICS: &Graphics = include_aseprite!("gfx/whitch_design.aseprite");
const TAG_MAP: &TagMap = GRAPHICS.tags();
const IDLE: &Tag = TAG_MAP.get("idle");

pub struct Player<'a> {
    object_controller: &'a OamManaged<'a>,
    pub position: Vector2D<u16>,
    sprite: Object<'a>,
    input: ButtonController,
}

impl<'a> Player<'a> {
    pub fn new(object_controller: &'a OamManaged<'a>) -> Self {
        Self {
            object_controller,
            position: (50u16, 50u16).into(),
            sprite: object_controller.object_sprite(IDLE.sprite(0)),
            input: ButtonController::new(),
        }
    }

    pub fn update(&mut self) {
        if self.input.is_pressed(Button::RIGHT) {
            self.position.x += 20;
        }
        if self.input.is_pressed(Button::LEFT) {
            self.position.x -= 20;
        }
        self.sprite
            .set_x(self.position.x)
            .set_y(self.position.y)
            .show();
        self.object_controller.commit();

        self.input.update();
    }
}
