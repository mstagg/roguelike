use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovesRandomly)]
pub fn random_movement(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut moving_entities = <(&mut Point, &MovesRandomly)>::query();

    moving_entities.iter_mut(ecs).for_each(|(pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(pos.x - 1, pos.y),
            1 => Point::new(pos.x + 1, pos.y),
            2 => Point::new(pos.x, pos.y - 1),
            _ => Point::new(pos.x, pos.y + 1),
        };
        if map.is_walkable(destination) {
            *pos = destination;
        }
    });
}
