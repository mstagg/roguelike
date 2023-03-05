use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] current_state: &mut TurnState) {
    let new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
    };
    *current_state = new_state;
}
