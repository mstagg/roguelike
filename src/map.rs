use crate::prelude::*;

const TILE_COUNT: usize = WINDOW_WIDTH * WINDOW_HEIGHT;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Hall,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; TILE_COUNT],
        }
    }

    pub fn tile_idx(&self, p: Point) -> usize {
        (p.y as usize * WINDOW_WIDTH) + p.x as usize
    }

    pub fn in_bounds(&self, p: Point) -> bool {
        p.x >= 0 && p.x < WINDOW_WIDTH as i32 && p.y >= 0 && p.y < WINDOW_HEIGHT as i32
    }

    pub fn is_walkable(&self, p: Point) -> bool {
        if let Some(idx) = self.try_point(p) {
            return match self.tiles[idx] {
                TileType::Floor => true,
                TileType::Hall => true,
                _ => false,
            };
        };
        false
    }

    pub fn try_point(&self, p: Point) -> Option<usize> {
        if self.in_bounds(p) {
            Some(self.tile_idx(p))
        } else {
            None
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);

        for x in camera.left_x..camera.right_x {
            for y in camera.top_y..camera.bottom_y {
                let tile = Point::new(x, y);
                if self.in_bounds(tile) {
                    let idx = self.tile_idx(tile);

                    match self.tiles[idx] {
                        TileType::Floor => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('.'),
                        ),
                        TileType::Hall => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('.'),
                        ),
                        TileType::Wall => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('#'),
                        ),
                    }
                }
            }
        }
    }
}
