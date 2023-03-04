use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Enemy)]
pub fn collisions(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut players = <&Point>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    let mut player_position = Point::zero();
    players.iter(ecs).for_each(|player_pos| {
        player_position = *player_pos;
    });

    enemies.iter(ecs).for_each(|(entity, enemy_pos)| {
        if player_position == *enemy_pos {
            commands.remove(*entity);
        }
    });
}
