use bevy::prelude::States;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    BeforeGame,
    XTurn,
    ZeroTurn,
    XWins,
    ZeroWins,
    Draw,
}