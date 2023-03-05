use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, spawn_point: Point) {
    ecs.push((
        Player,
        Point::new(spawn_point.x, spawn_point.y),
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, spawn_point: Point) {
    let mut rng = RandomNumberGenerator::new();
    ecs.push((
        Enemy,
        MovesRandomly,
        Point::new(spawn_point.x, spawn_point.y),
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: match rng.range(0, 4) {
                0 => to_cp437('E'),
                1 => to_cp437('O'),
                2 => to_cp437('o'),
                _ => to_cp437('g'),
            },
        },
    ));
}
