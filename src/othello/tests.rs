use super::*;
use errors::OthelloResult;

struct MockDatabase {
    state: GameStateEntity
}

impl MockDatabase {
    fn new() -> MockDatabase {
        use std::collections::HashMap;

        let mut board = HashMap::new();
        board.insert(Point::new(3, 3), Player::Black);
        board.insert(Point::new(4, 4), Player::Black);
        board.insert(Point::new(3, 4), Player::White);
        board.insert(Point::new(4, 3), Player::White);

        MockDatabase {
            state: GameStateEntity::new(board, Player::Black, None),
        }
    }
}

impl database::DatabaseConnection for MockDatabase {
    fn load_state(&self) -> Result<GameStateEntity> {
        Ok(self.state.clone())
    }

    fn save_state(&mut self, state: GameStateEntity) -> OthelloResult<()> {
        self.state = state;
        Ok(())
    }
}

#[test]
fn exist_adjacent_enemy() {
    let mut board = GameBoard::new();
    board.insert(Point::new(4, 4), Player::Black);
    board.insert(Point::new(3, 4), Player::White);

    let rule = MustExistAdjacentEnemy;
    let state = GameStateEntity::new(board, Player::Black, None);

    assert!(rule.is_valid(&Point::new(2, 4), &state));
}

#[test]
fn test_if_enclosing_friendly() {
    let mut board = GameBoard::new();
    board.insert(Point::new(4, 4), Player::Black); // Center
    board.insert(Point::new(3, 4), Player::White); // Left
    board.insert(Point::new(5, 4), Player::White); // Right
    board.insert(Point::new(4, 5), Player::White); // Bottom
    board.insert(Point::new(4, 3), Player::White); // Top
    board.insert(Point::new(3, 3), Player::White); // Top-left
    board.insert(Point::new(5, 3), Player::White); // Top-right
    board.insert(Point::new(3, 5), Player::White); // Bottom-left
    board.insert(Point::new(5, 5), Player::White); // Bottom-right


    let rule = MustExistAdjacentEnemy;
    let state = GameStateEntity::new(board, Player::Black, None);

    let ok_placements = vec![
        Point::new(2, 2), Point::new(4, 2), Point::new(6, 2),
        Point::new(2, 4),                   Point::new(6, 4),
        Point::new(2, 6), Point::new(4, 6), Point::new(6, 6)
    ];

    let bad_placement = Point::new(1, 4);

    assert!(ok_placements.iter().all(|pos| rule.is_valid(pos, &state)));
    assert!(!rule.is_valid(&bad_placement, &state));
}

#[test]
fn test_capture() {
    let mut logic = Logic::new(RuleBook::new(), MockDatabase::new());
    let new_state = logic.place_tile(Point::new(5, 3)).unwrap();

    let black_tile_count = new_state.board.values()
        .filter(|&val| *val == Player::Black)
        .count();

    let white_tile_count = new_state.board.values()
        .filter(|&val| *val == Player::White)
        .count();

    assert_eq!(4, black_tile_count);
    assert_eq!(1, white_tile_count);
}
