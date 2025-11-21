use crate::components::{GameField, PlayableCellState};

const WIN_PATTERNS: [[(usize, usize); 3]; 8] = [
    // Горизонтали
    [(0,0), (0,1), (0,2)],
    [(1,0), (1,1), (1,2)],
    [(2,0), (2,1), (2,2)],
    // Вертикали
    [(0,0), (1,0), (2,0)],
    [(0,1), (1,1), (2,1)],
    [(0,2), (1,2), (2,2)],
    // Диагонали
    [(0,0), (1,1), (2,2)],
    [(0,2), (1,1), (2,0)],
];

pub fn check_winner(game_field: &GameField) -> Option<PlayableCellState> {
    // Проверка победы
    for &[a, b, c] in &WIN_PATTERNS {
        let (ax, ay) = a;
        let (bx, by) = b;
        let (cx, cy) = c;

        if game_field.field[ax][ay] != PlayableCellState::Empty
            && game_field.field[ax][ay] == game_field.field[bx][by]
            && game_field.field[bx][by] == game_field.field[cx][cy]
        {
            return Some(game_field.field[ax][ay]);
        }
    }

    // Проверка ничьи (все клетки заполнены и нет победителя)
    if is_draw(game_field) {
        return Some(PlayableCellState::Empty); // или специальное значение для ничьи
    }

    None
}

fn is_draw(game_field: &GameField) -> bool {
    // Проверяем, что все клетки заполнены (нет пустых)
    for row in &game_field.field {
        for cell in row {
            if *cell == PlayableCellState::Empty {
                return false;
            }
        }
    }
    true
}