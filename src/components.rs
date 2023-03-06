pub use crate::prelude::*;

pub struct MoveIntentMessage {
    pub entity: Entity,
    pub destination: Point,
}

pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

pub struct Player;
pub struct Enemy;

pub struct MovesRandomly;
