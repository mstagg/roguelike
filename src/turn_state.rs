#[derive(Clone)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    EnemyTurn,
}
