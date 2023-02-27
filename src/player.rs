use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(p: Point) -> Self {
        Self { position: p }
    }

    pub fn poll_input_and_update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };
            let new_pos = self.position + delta;
            if map.is_walkable(new_pos) {
                self.position = new_pos;
            }
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            YELLOW,
            BLACK,
            to_cp437('@'),
        );
    }
}
