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
            return matches!(self.tiles[idx], TileType::Floor | TileType::Hall);
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
}
