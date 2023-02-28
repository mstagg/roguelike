use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(p: Point) -> Self {
        Self { position: p }
    }

    pub fn poll_input_and_update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
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
                camera.move_to_point(new_pos);
            }
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(1);
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }
}
