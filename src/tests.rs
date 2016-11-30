use super::*;

#[test]
fn exist_adjacent_enemy() {
    let mut board = GameBoard::new();
    board.insert(Point::new(4, 4), Color::Black);
    board.insert(Point::new(3, 4), Color::White);

    let rule = MustExistAdjacentEnemy;
    let state = GameStateEntity::new(board, Color::Black, None);

    assert!(rule.is_valid(&Point::new(2, 4), &state));
}

#[test]
fn test_if_enclosing_friendly() {
    let mut board = GameBoard::new();
    board.insert(Point::new(4, 4), Color::Black); // Center
    board.insert(Point::new(3, 4), Color::White); // Left
    board.insert(Point::new(5, 4), Color::White); // Right
    board.insert(Point::new(4, 5), Color::White); // Bottom
    board.insert(Point::new(4, 3), Color::White); // Top
    board.insert(Point::new(3, 3), Color::White); // Top-left
    board.insert(Point::new(5, 3), Color::White); // Top-right
    board.insert(Point::new(3, 5), Color::White); // Bottom-left
    board.insert(Point::new(5, 5), Color::White); // Bottom-right


    let rule = MustExistAdjacentEnemy;
    let state = GameStateEntity::new(board, Color::Black, None);

    let ok_placements = vec![
        Point::new(2, 2), Point::new(4, 2), Point::new(6, 2),
        Point::new(2, 4),                   Point::new(6, 4),
        Point::new(2, 6), Point::new(4, 6), Point::new(6, 6)
    ];

    let bad_placement = Point::new(1, 4);

    assert!(ok_placements.iter().all(|pos| rule.is_valid(pos, &state)));
    assert!(!rule.is_valid(&bad_placement, &state));
}
