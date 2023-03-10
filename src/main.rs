mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const WINDOW_WIDTH: usize = 80;
    pub const WINDOW_HEIGHT: usize = 50;
    pub const VIEW_WIDTH: usize = WINDOW_WIDTH / 2;
    pub const VIEW_HEIGHT: usize = WINDOW_HEIGHT / 2;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mb = MapBuilder::new();

        spawn_player(&mut ecs, mb.player_start);
        mb.rooms
            .iter()
            .skip(1)
            .map(|room| room.center())
            .for_each(|room_center| {
                spawn_monster(&mut ecs, room_center);
            });

        resources.insert(mb.map);
        resources.insert(Camera::new(mb.player_start));
        resources.insert(TurnState::AwaitingInput);
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_sceduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);

        let current_state = self
            .resources
            .get::<TurnState>()
            .expect("Current State Not Found")
            .clone();

        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::EnemyTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
        }

        render_draw_buffer(ctx).expect("Render error");
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
