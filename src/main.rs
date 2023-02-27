mod map;
mod map_builder;
mod player;

mod prelude {
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use bracket_lib::prelude::*;
    pub const WINDOW_WIDTH: usize = 80;
    pub const WINDOW_HEIGHT: usize = 50;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mb = MapBuilder::new();
        Self {
            map: mb.map,
            player: Player::new(mb.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.player.poll_input_and_update(ctx, &self.map);

        ctx.cls();
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() {
    let window = BTermBuilder::simple80x50()
        .with_title("Roguelike")
        .with_fps_cap(30.0)
        .build()
        .unwrap();

    main_loop(window, State::new()).unwrap();
}
