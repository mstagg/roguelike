use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(player_pos: Point) -> Self {
        Self {
            left_x: player_pos.x - VIEW_WIDTH as i32 / 2,
            right_x: player_pos.x + VIEW_WIDTH as i32 / 2,
            top_y: player_pos.y - VIEW_HEIGHT as i32 / 2,
            bottom_y: player_pos.y + VIEW_HEIGHT as i32 / 2,
        }
    }

    pub fn move_to_point(&mut self, pos: Point) {
        self.left_x = pos.x - VIEW_WIDTH as i32 / 2;
        self.right_x = pos.x + VIEW_WIDTH as i32 / 2;
        self.top_y = pos.y - VIEW_HEIGHT as i32 / 2;
        self.bottom_y = pos.y + VIEW_HEIGHT as i32 / 2;
    }
}
