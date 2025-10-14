use rust_chess_core::board::Board;
use rust_chess_core::board::PieceColor::{Black, White};
use rust_chess_core::game::{Game, GameHistory};
use rust_chess_core::r#move::Move;
use rust_chess_core::pos::Pos;

fn assert_eq_move_arrays(a: &[Move], b: &[Move]) -> Result<(), String> {
    assert_eq!(
        a.len(),
        b.len(),
        "Arrays have different lengths:\n Expected: {:?}\r\n Actual:   {:?}",
        a,
        b
    );
    // Assuming that all moves in arrays are different, if we just check that all moves from a are in b, then arrays are equal
    for mv in a {
        assert!(
            b.contains(mv),
            "Move {:?} not found in second array:\n Expected: {:?}\r\n Actual: {:?}",
            mv,
            a,
            b
        );
    }
    Ok(())
}

#[test]
fn bishop_moves() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  -- :: -- :: bK :: -- ::
        7  :: -- :: -- :: bp bp --
        6  -- :: -- :: -- :: -- ::
        5  :: -- :: -- :: -- :: --
        4  -- :: -- :: -- :: -- ::
        3  :: -- :: -- :: wB :: --
        2  -- :: -- :: -- :: -- ::
        1  :: -- :: -- wK -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let game = Game::from_board(board, White);

    let expected_moves = [
        Move::from_long_notation("f3e2"),
        Move::from_long_notation("f3d1"),
        Move::from_long_notation("f3g2"),
        Move::from_long_notation("f3h1"),
        Move::from_long_notation("f3e4"),
        Move::from_long_notation("f3d5"),
        Move::from_long_notation("f3c6"),
        Move::from_long_notation("f3b7"),
        Move::from_long_notation("f3a8"),
        Move::from_long_notation("f3g4"),
        Move::from_long_notation("f3h5"),
    ];

    assert_eq_move_arrays(
        &expected_moves,
        game.get_moves_from_pos(Pos::from_notation("f3")?),
    )
}

#[test]
fn rook_moves() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  -- :: -- :: bK :: -- ::
        7  :: -- :: -- :: bp bp --
        6  -- :: -- :: -- :: -- ::
        5  :: -- :: -- :: -- :: --
        4  -- :: wR :: -- :: -- ::
        3  :: -- :: -- :: -- :: --
        2  -- :: -- :: -- :: -- ::
        1  :: -- :: -- wK -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let game = Game::from_board(board, White);

    let expected_moves = [
        Move::from_long_notation("c4b4"),
        Move::from_long_notation("c4a4"),
        Move::from_long_notation("c4d4"),
        Move::from_long_notation("c4e4"),
        Move::from_long_notation("c4f4"),
        Move::from_long_notation("c4g4"),
        Move::from_long_notation("c4h4"),
        Move::from_long_notation("c4c3"),
        Move::from_long_notation("c4c2"),
        Move::from_long_notation("c4c1"),
        Move::from_long_notation("c4c5"),
        Move::from_long_notation("c4c6"),
        Move::from_long_notation("c4c7"),
        Move::from_long_notation("c4c8"),
    ];

    assert_eq_move_arrays(
        &expected_moves,
        game.get_moves_from_pos(Pos::from_notation("c4")?),
    )
}

#[test]
fn knight_moves() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  -- :: -- :: bK :: -- ::
        7  :: -- :: -- :: bp bp --
        6  -- :: -- :: -- :: -- ::
        5  :: -- :: -- :: -- :: --
        4  -- :: wN :: -- :: -- ::
        3  :: -- :: -- :: -- :: --
        2  -- :: -- :: -- :: -- ::
        1  :: -- :: -- wK -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let game = Game::from_board(board, White);

    let expected_moves = [
        Move::from_long_notation("c4b6"),
        Move::from_long_notation("c4a5"),
        Move::from_long_notation("c4a3"),
        Move::from_long_notation("c4b2"),
        Move::from_long_notation("c4d2"),
        Move::from_long_notation("c4e3"),
        Move::from_long_notation("c4e5"),
        Move::from_long_notation("c4d6"),
    ];

    assert_eq_move_arrays(
        &expected_moves,
        game.get_moves_from_pos(Pos::from_notation("c4")?),
    )
}

#[test]
fn queen_moves() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  -- :: -- :: bK :: -- ::
        7  :: -- :: -- :: -- bp --
        6  -- :: -- :: -- :: -- ::
        5  :: -- :: -- :: -- :: --
        4  -- :: wQ :: -- :: -- ::
        3  :: -- :: -- :: -- :: --
        2  -- :: -- :: -- :: -- ::
        1  :: -- :: -- wK -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let game = Game::from_board(board, White);

    let expected_moves = [
        // rook moves
        Move::from_long_notation("c4b4"),
        Move::from_long_notation("c4a4"),
        Move::from_long_notation("c4d4"),
        Move::from_long_notation("c4e4"),
        Move::from_long_notation("c4f4"),
        Move::from_long_notation("c4g4"),
        Move::from_long_notation("c4h4"),
        Move::from_long_notation("c4c3"),
        Move::from_long_notation("c4c2"),
        Move::from_long_notation("c4c1"),
        Move::from_long_notation("c4c5"),
        Move::from_long_notation("c4c6"),
        Move::from_long_notation("c4c7"),
        Move::from_long_notation("c4c8"),
        // bishop moves
        Move::from_long_notation("c4b3"),
        Move::from_long_notation("c4a2"),
        Move::from_long_notation("c4d3"),
        Move::from_long_notation("c4e2"),
        Move::from_long_notation("c4f1"),
        Move::from_long_notation("c4b5"),
        Move::from_long_notation("c4a6"),
        Move::from_long_notation("c4d5"),
        Move::from_long_notation("c4e6"),
        Move::from_long_notation("c4f7"),
        Move::from_long_notation("c4g8"),
    ];

    assert_eq_move_arrays(
        &expected_moves,
        game.get_moves_from_pos(Pos::from_notation("c4")?),
    )
}

#[test]
fn simple_king_moves() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  -- :: -- :: bK :: -- ::
        7  :: -- :: -- :: -- bp --
        6  -- :: -- :: -- :: -- ::
        5  :: -- :: -- :: -- :: --
        4  -- :: wp :: -- :: -- ::
        3  :: -- :: -- wK -- :: --
        2  -- :: -- :: -- :: -- ::
        1  :: -- :: -- :: -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let game = Game::from_board(board, White);

    let expected_moves = [
        Move::from_long_notation("e3d2"),
        Move::from_long_notation("e3d3"),
        Move::from_long_notation("e3d4"),
        Move::from_long_notation("e3e2"),
        Move::from_long_notation("e3e4"),
        Move::from_long_notation("e3f2"),
        Move::from_long_notation("e3f3"),
        Move::from_long_notation("e3f4"),
    ];

    assert_eq_move_arrays(
        &expected_moves,
        game.get_moves_from_pos(Pos::from_notation("e3")?),
    )
}

#[test]
fn king_castles() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  bR :: -- :: bK :: -- bR
        7  :: -- :: -- :: -- bp --
        6  -- :: -- :: -- :: -- ::
        5  wR -- :: -- wB -- :: wR
        4  -- :: wp :: -- :: -- ::
        3  :: -- :: -- wK -- :: --
        2  -- :: -- :: -- :: -- ::
        1  :: -- :: -- :: -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let game = Game::from_board(board, Black);

    let expected_moves = [
        // regular moves
        Move::from_long_notation("e8d8"),
        Move::from_long_notation("e8d7"),
        Move::from_long_notation("e8e7"),
        Move::from_long_notation("e8f7"),
        Move::from_long_notation("e8f8"),
        // castles
        Move::from_long_notation("e8c8"),
        Move::from_long_notation("e8g8"),
    ];

    assert_eq_move_arrays(
        &expected_moves,
        game.get_moves_from_pos(Pos::from_notation("e8")?),
    )
}

#[test]
fn king_castle_only_g() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  -- :: -- :: bK :: -- bR
        7  bR -- :: -- :: -- bp --
        6  -- :: -- :: -- :: -- ::
        5  :: -- :: -- :: -- :: wR
        4  -- :: wp :: -- :: -- ::
        3  :: -- :: -- wK -- :: --
        2  -- :: -- :: -- :: -- ::
        1  :: -- :: -- :: -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let game = Game::from_board(board, Black);

    let expected_moves = [
        // regular moves
        Move::from_long_notation("e8d8"),
        Move::from_long_notation("e8d7"),
        Move::from_long_notation("e8e7"),
        Move::from_long_notation("e8f7"),
        Move::from_long_notation("e8f8"),
        // castles
        Move::from_long_notation("e8g8"),
    ];

    assert_eq_move_arrays(
        &expected_moves,
        game.get_moves_from_pos(Pos::from_notation("e8")?),
    )
}

#[test]
fn king_castle_only_c() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  bR :: -- :: bK :: -- ::
        7  :: -- :: -- :: -- bp --
        6  -- :: -- :: -- :: -- ::
        5  wR wR :: -- :: -- :: --
        4  -- :: wp :: -- :: -- ::
        3  :: -- :: -- wK -- :: --
        2  -- :: -- :: -- :: -- ::
        1  :: -- :: -- :: -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let game = Game::from_board(board, Black);

    let expected_moves = [
        // regular moves
        Move::from_long_notation("e8d8"),
        Move::from_long_notation("e8d7"),
        Move::from_long_notation("e8e7"),
        Move::from_long_notation("e8f7"),
        Move::from_long_notation("e8f8"),
        // castles
        Move::from_long_notation("e8c8"),
    ];

    assert_eq_move_arrays(
        &expected_moves,
        game.get_moves_from_pos(Pos::from_notation("e8")?),
    )
}

#[test]
fn king_cant_castle_when_king_was_moved() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  bR :: -- :: bK :: -- bR
        7  :: -- :: -- :: -- bp --
        6  -- :: -- :: -- :: -- ::
        5  :: -- :: -- :: -- :: --
        4  -- :: wp :: -- :: -- ::
        3  :: -- :: -- wK -- :: --
        2  -- :: -- :: -- :: -- ::
        1  :: -- :: -- :: -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let history = GameHistory::with_moves(vec![
        Move::from_long_notation("e8d8"),
        Move::from_long_notation("d4e3"),
        Move::from_long_notation("d8e8"),
        Move::from_long_notation("e3d4"),
    ]);
    let game = Game::from_board_with_history(board, Black, history);

    let expected_moves = [
        // regular moves
        Move::from_long_notation("e8d8"),
        Move::from_long_notation("e8d7"),
        Move::from_long_notation("e8e7"),
        Move::from_long_notation("e8f7"),
        Move::from_long_notation("e8f8"),
    ];

    assert_eq_move_arrays(
        &expected_moves,
        game.get_moves_from_pos(Pos::from_notation("e8")?),
    )
}

#[test]
fn king_cant_castle_when_rook_was_moved() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  bR :: -- :: bK :: -- bR
        7  :: -- :: -- :: -- bp --
        6  -- :: -- :: -- :: -- ::
        5  :: -- :: -- :: -- :: --
        4  -- :: wp :: -- :: -- ::
        3  :: -- :: -- wK -- :: --
        2  -- :: -- :: -- :: -- ::
        1  :: -- :: -- :: -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let history = GameHistory::with_moves(vec![
        Move::from_long_notation("a8a1"),
        Move::from_long_notation("d4e3"),
        Move::from_long_notation("a1a8"),
        Move::from_long_notation("e3d4"),
    ]);
    let game = Game::from_board_with_history(board, Black, history);

    // Can only move to rook which hasn't moved
    let expected_moves = [
        // regular moves
        Move::from_long_notation("e8d8"),
        Move::from_long_notation("e8d7"),
        Move::from_long_notation("e8e7"),
        Move::from_long_notation("e8f7"),
        Move::from_long_notation("e8f8"),
        // castles
        Move::from_long_notation("e8g8"),
    ];

    assert_eq_move_arrays(
        &expected_moves,
        game.get_moves_from_pos(Pos::from_notation("e8")?),
    )
}

const KING_CANT_CASTLE_WHEN_UNDER_CHECK_BOARDS: [&str; 3] = [
    "8  bR :: -- :: bK :: -- ::
     7  :: -- :: -- :: -- bp --
     6  -- :: -- :: -- :: -- ::
     5  :: -- :: -- :: -- :: --
     4  -- :: wp :: wR :: -- ::
     3  :: -- :: -- wK -- :: --
     2  -- :: -- :: -- :: -- ::
     1  :: -- :: -- :: -- :: --
         a  b  c  d  e  f  g  h",
    "8  bR :: -- :: bK :: -- ::
     7  :: -- :: -- :: -- bp --
     6  -- :: -- :: -- :: -- ::
     5  :: -- :: wR :: -- :: --
     4  -- :: wp :: -- :: -- ::
     3  :: -- :: -- wK -- :: --
     2  -- :: -- :: -- :: -- ::
     1  :: -- :: -- :: -- :: --
         a  b  c  d  e  f  g  h",
    "8  bR :: -- :: bK :: -- ::
     7  :: -- :: -- :: -- bp --
     6  -- :: -- :: -- :: -- ::
     5  :: -- wR -- :: -- :: --
     4  -- :: wp :: -- :: -- ::
     3  :: -- :: -- wK -- :: --
     2  -- :: -- :: -- :: -- ::
     1  :: -- :: -- :: -- :: --
         a  b  c  d  e  f  g  h",
];

#[test]
fn king_cant_castle_when_king_under_check() -> Result<(), String> {
    for board_str in KING_CANT_CASTLE_WHEN_UNDER_CHECK_BOARDS {
        let board = Board::from_string(board_str)?;
        let game = Game::from_board(board, Black);

        let forbidden_move = Move::from_long_notation("e8c8");

        assert!(
            !game
                .get_moves_from_pos(Pos::from_notation("e8")?)
                .contains(&forbidden_move)
        );
    }
    Ok(())
}

#[test]
fn king_can_castle_long() -> Result<(), String> {
    let board = Board::from_string(
    "8  bR bN -- bK -- :: -- bR
           7  bp -- :: -- :: bp bp bp
           6  bB bp bp bB bp :: -- ::
           5  :: -- :: bp :: -- :: --
           4  wN :: -- wp -- :: -- wp
           3  wp -- :: -- wp wN :: --
           2  -- wp wp :: wp :: wp ::
           1  wR -- :: -- wK wB :: wR
               a  b  c  d  e  f  g  h"
    )?;
    let game = Game::from_board(board, White);
    let expected_move = Move::from_long_notation("e1c1");
    
    let moves_from_pos = game
        .get_moves_from_pos(Pos::from_notation("e1")?);
    
    assert!(
        moves_from_pos.contains(&expected_move),
            "{:?} didn't contain {}", moves_from_pos, expected_move);
    
    Ok(())
}

// TODO: Check for short castle when under attack, when spaces between are occupied