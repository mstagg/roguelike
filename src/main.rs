mod camera;
mod map;
mod map_builder;
mod player;

mod prelude {
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use bracket_lib::prelude::*;
    pub const WINDOW_WIDTH: usize = 80;
    pub const WINDOW_HEIGHT: usize = 50;
    pub const VIEW_WIDTH: usize = WINDOW_WIDTH / 2;
    pub const VIEW_HEIGHT: usize = WINDOW_HEIGHT / 2;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        let mb = MapBuilder::new();
        Self {
            map: mb.map,
            player: Player::new(mb.player_start),
            camera: Camera::new(mb.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.player
            .poll_input_and_update(ctx, &self.map, &mut self.camera);

        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}

fn main() {
    let window = BTermBuilder::new()
        .with_title("roguelike")
        .with_fps_cap(30.0)
        .with_dimensions(VIEW_WIDTH, VIEW_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("res/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(VIEW_WIDTH, VIEW_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(VIEW_WIDTH, VIEW_HEIGHT, "dungeonfont.png")
        .build()
        .expect("Failed to build BTerm");

    main_loop(window, State::new()).unwrap();
}
