use crate::prelude::*;
use std::cmp::{max, min};

const ROOM_COUNT: usize = 15;
const ROOM_MIN_SIZE: usize = 3;
const ROOM_MAX_SIZE: usize = 10;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new() -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        mb.fill_map(TileType::Wall);
        mb.build_random_rooms();
        mb.build_hallways();
        mb.player_start = mb.rooms[0].center();
        mb
    }

    fn fill_map(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self) {
        while self.rooms.len() < ROOM_COUNT {
            let mut rng = RandomNumberGenerator::new();
            let w = rng.range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let h = rng.range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let x = rng.range(0, WINDOW_WIDTH - w - 1);
            let y = rng.range(0, WINDOW_HEIGHT - h - 1);
            let room = Rect::with_size(x, y, w, h);

            let mut intersect = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    intersect = true;
                    break;
                }
            }

            if !intersect {
                room.for_each(|tile| {
                    if let Some(idx) = self.map.try_point(tile) {
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }

    fn build_vertical_hallway(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_point(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Hall;
            }
        }
    }

    fn build_horizontal_hallway(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_point(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Hall;
            }
        }
    }

    fn build_hallways(&mut self) {
        let mut rng = RandomNumberGenerator::new();
        let mut rooms = self.rooms.clone();

        // Sort rooms by center so they are as close to each other as possible.
        // This helps keep hallways close, so they dont snake all over the map.
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.build_horizontal_hallway(prev.x, new.x, prev.y);
                self.build_vertical_hallway(prev.y, new.y, new.x);
            } else {
                self.build_vertical_hallway(prev.y, new.y, prev.x);
                self.build_horizontal_hallway(prev.x, new.x, new.y);
            }
        }
    }
}
