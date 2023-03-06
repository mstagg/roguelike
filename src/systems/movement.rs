use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    move_message: &MoveIntentMessage,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.is_walkable(move_message.destination) {
        // if destination is valid, add it to the entity, it will override the old one
        commands.add_component(move_message.entity, move_message.destination);

        // if the entity is a player, move the camera to center on it
        if ecs
            .entry_ref(move_message.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.move_to_point(move_message.destination);
        }
    }
    // remove the message entity, otherwise it will be processed again on the next tick
    commands.remove(*entity);
}
