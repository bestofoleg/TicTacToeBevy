use bevy::prelude::Component;

#[derive(Component)]
pub struct PlayableCell {
    pub state: PlayableCellState,
    pub x_index: i32,
    pub y_index: i32,
}

#[derive(Component)]
pub struct GameField {
    pub field: [[PlayableCellState; 3]; 3],
}

impl GameField {
    pub fn default() -> Self {
        Self {
            field: [
                [PlayableCellState::Empty, PlayableCellState::Empty, PlayableCellState::Empty],
                [PlayableCellState::Empty, PlayableCellState::Empty, PlayableCellState::Empty],
                [PlayableCellState::Empty, PlayableCellState::Empty, PlayableCellState::Empty],
            ]
        }
    }

    pub fn put_on_field(
        &mut self,
        x: usize,
        y: usize,
        state: PlayableCellState
    ) {
        let current_state = self.field[x][y];
        if current_state == PlayableCellState::Empty {
            self.field[x][y] = state;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayableCellState {
    X,
    Zero,
    Empty,
}