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
fn pawn_move() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  -- :: -- :: bK :: -- ::
        7  bp -- bp -- :: bp :: --
        6  wp :: -- :: -- :: bp ::
        5  :: -- wp -- bp -- :: --
        4  -- :: bp :: wp :: -- ::
        3  :: -- :: -- :: wB :: --
        2  bp :: -- :: -- :: -- ::
        1  :: -- :: -- wK -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let game = Game::from_board(board, Black);

    let expected_f7_moves = [
        Move::from_long_notation("f7f6"),
        Move::from_long_notation("f7f5"),
    ];
    let expected_g6_moves = [Move::from_long_notation("g6g5")];
    let expected_c4_moves = [Move::from_long_notation("c4c3")];
    let expected_a2_moves = [
        Move::from_long_notation("a2a1q"), // we only check queen promotion here, because other promotions are allowed if queen is in possible moves
    ];
    let expected_a7_moves = [];
    let expected_c7_moves = [Move::from_long_notation("c7c6")];
    let expected_e5_moves = [];

    assert_eq_move_arrays(
        &expected_f7_moves,
        game.get_moves_from_pos(Pos::from_notation("f7")?),
    )?;
    assert_eq_move_arrays(
        &expected_g6_moves,
        game.get_moves_from_pos(Pos::from_notation("g6")?),
    )?;
    assert_eq_move_arrays(
        &expected_c4_moves,
        game.get_moves_from_pos(Pos::from_notation("c4")?),
    )?;
    assert_eq_move_arrays(
        &expected_a2_moves,
        game.get_moves_from_pos(Pos::from_notation("a2")?),
    )?;
    assert_eq_move_arrays(
        &expected_a7_moves,
        game.get_moves_from_pos(Pos::from_notation("a7")?),
    )?;
    assert_eq_move_arrays(
        &expected_c7_moves,
        game.get_moves_from_pos(Pos::from_notation("c7")?),
    )?;
    assert_eq_move_arrays(
        &expected_e5_moves,
        game.get_moves_from_pos(Pos::from_notation("e5")?),
    )
}

#[test]
fn pawn_captures() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  -- :: -- :: bK :: -- ::
        7  :: -- bp -- bp -- :: --
        6  -- :: -- wp -- :: -- ::
        5  :: -- :: -- :: bp :: --
        4  -- bp -- :: wp :: wp ::
        3  :: wp wp -- :: -- :: --
        2  -- bp -- :: -- :: -- ::
        1  wR -- :: -- wK -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let game = Game::from_board(board, Black);

    let expected_c5_moves = [
        Move::from_long_notation("c7c6"),
        Move::from_long_notation("c7c5"),
        Move::from_long_notation("c7d6"),
    ];
    let expected_e5_moves = [
        Move::from_long_notation("e7e6"),
        Move::from_long_notation("e7e5"),
        Move::from_long_notation("e7d6"),
    ];
    let expected_f4_moves = [
        Move::from_long_notation("f5f4"),
        Move::from_long_notation("f5e4"),
        Move::from_long_notation("f5g4"),
    ];
    let expected_b4_moves = [
        Move::from_long_notation("b4c3"),
    ];
    let expected_b2_moves = [
        Move::from_long_notation("b2a1q"),
        Move::from_long_notation("b2b1q"),
    ];

    assert_eq_move_arrays(
        &expected_c5_moves,
        game.get_moves_from_pos(Pos::from_notation("c7")?),
    )?;
    assert_eq_move_arrays(
        &expected_e5_moves,
        game.get_moves_from_pos(Pos::from_notation("e7")?),
    )?;
    assert_eq_move_arrays(
        &expected_f4_moves,
        game.get_moves_from_pos(Pos::from_notation("f5")?),
    )?;
    assert_eq_move_arrays(
        &expected_b4_moves,
        game.get_moves_from_pos(Pos::from_notation("b4")?),
    )?;
    assert_eq_move_arrays(
        &expected_b2_moves,
        game.get_moves_from_pos(Pos::from_notation("b2")?),
    )?;
    Ok(())
}


#[test]
fn pawn_en_passant() ->  Result<(), String> {
    let board = Board::from_string(
        "
        8  -- :: -- bK -- :: -- ::
        7  :: -- :: -- bp -- bp --
        6  -- :: -- :: -- :: -- ::
        5  :: -- :: -- :: wp :: --
        4  -- :: -- wp -- :: -- ::
        3  :: -- :: -- :: -- :: --
        2  -- :: -- :: wp :: -- ::
        1  :: -- :: wK :: -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let mut game = Game::from_board(board, Black);

    let mv = Move::from_long_notation("e7e5");
    game.make_move(&mv)?;

    let expected_moves = [
        Move::from_long_notation("f5e6"), // en passant
        Move::from_long_notation("f5f6"),
    ];
    assert_eq_move_arrays(
        &expected_moves,
        game.get_moves_from_pos(Pos::from_notation("f5")?),
    )?;

    let mv = Move::from_long_notation("d4d5");
    game.make_move(&mv)?;

    let mv =  Move::from_long_notation("g7g5");
    game.make_move(&mv)?;

    let expected_moves = [
        Move::from_long_notation("f5g6"), // en passant
        Move::from_long_notation("f5f6"),
    ];
    assert_eq_move_arrays(
        &expected_moves,
        game.get_moves_from_pos(Pos::from_notation("f5")?),
    )?;

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
        // bottom left
        Move::from_long_notation("f3e2"),
        Move::from_long_notation("f3d1"),
        // bottom right
        Move::from_long_notation("f3g2"),
        Move::from_long_notation("f3h1"),
        // top left
        Move::from_long_notation("f3e4"),
        Move::from_long_notation("f3d5"),
        Move::from_long_notation("f3c6"),
        Move::from_long_notation("f3b7"),
        Move::from_long_notation("f3a8"),
        // top right
        Move::from_long_notation("f3g4"),
        Move::from_long_notation("f3h5"),
    ];

    assert_eq_move_arrays(
        &expected_moves,
        game.get_moves_from_pos(Pos::from_notation("f3")?),
    )
}

#[test]
fn bishop_captures() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  -- :: -- :: bK :: -- ::
        7  :: -- :: -- :: bp bp --
        6  -- :: bp :: -- :: -- ::
        5  :: -- :: -- :: -- :: bp
        4  -- :: -- :: -- :: -- ::
        3  :: -- :: -- :: wB :: --
        2  -- :: -- :: -- :: bR ::
        1  :: -- :: bN wK -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let game = Game::from_board(board, White);

    let expected_moves = [
        // bottom left
        Move::from_long_notation("f3e2"),
        Move::from_long_notation("f3d1"),
        // bottom right
        Move::from_long_notation("f3g2"),
        // top left
        Move::from_long_notation("f3e4"),
        Move::from_long_notation("f3d5"),
        Move::from_long_notation("f3c6"),
        // top right
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
        // left
        Move::from_long_notation("c4b4"),
        Move::from_long_notation("c4a4"),
        // right
        Move::from_long_notation("c4d4"),
        Move::from_long_notation("c4e4"),
        Move::from_long_notation("c4f4"),
        Move::from_long_notation("c4g4"),
        Move::from_long_notation("c4h4"),
        // down
        Move::from_long_notation("c4c3"),
        Move::from_long_notation("c4c2"),
        Move::from_long_notation("c4c1"),
        // up
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
fn rook_captures() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  -- :: -- :: bK :: -- ::
        7  :: -- bp -- :: bp bp --
        6  -- :: -- :: -- :: -- ::
        5  :: -- :: -- :: -- :: --
        4  -- bN wR :: -- bQ -- ::
        3  :: -- :: -- :: -- :: --
        2  -- :: bp :: -- :: -- ::
        1  :: -- :: -- wK -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let game = Game::from_board(board, White);

    let expected_moves = [
        // left
        Move::from_long_notation("c4b4"),
        // right
        Move::from_long_notation("c4d4"),
        Move::from_long_notation("c4e4"),
        Move::from_long_notation("c4f4"),
        // down
        Move::from_long_notation("c4c3"),
        Move::from_long_notation("c4c2"),
        // up
        Move::from_long_notation("c4c5"),
        Move::from_long_notation("c4c6"),
        Move::from_long_notation("c4c7"),
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
fn knight_captures() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  -- :: -- :: bK :: -- ::
        7  :: -- :: -- :: bp bp --
        6  -- bQ -- wp -- :: -- ::
        5  bp -- :: -- :: -- :: --
        4  -- :: wN :: -- :: -- ::
        3  :: -- :: -- bp -- :: --
        2  -- bp -- bR -- :: -- ::
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
fn queen_captures() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  -- :: -- :: bK :: -- ::
        7  :: -- :: -- :: bp bp --
        6  bp :: -- :: -- :: -- ::
        5  :: bp :: -- :: -- :: --
        4  bp :: wQ bp bp :: -- ::
        3  :: bp :: -- :: -- :: --
        2  bp :: -- :: -- :: -- ::
        1  :: -- :: -- wK bB :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let game = Game::from_board(board, White);

    let expected_moves = [
        // rook moves
        Move::from_long_notation("c4b4"),
        Move::from_long_notation("c4a4"),
        Move::from_long_notation("c4d4"),
        Move::from_long_notation("c4c3"),
        Move::from_long_notation("c4c2"),
        Move::from_long_notation("c4c1"),
        Move::from_long_notation("c4c5"),
        Move::from_long_notation("c4c6"),
        Move::from_long_notation("c4c7"),
        Move::from_long_notation("c4c8"),
        // bishop moves
        Move::from_long_notation("c4b3"),
        Move::from_long_notation("c4d3"),
        Move::from_long_notation("c4e2"),
        Move::from_long_notation("c4f1"),
        Move::from_long_notation("c4b5"),
        Move::from_long_notation("c4d5"),
        Move::from_long_notation("c4e6"),
        Move::from_long_notation("c4f7"),
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
fn king_captures() -> Result<(), String> {
    let board = Board::from_string(
        "
        8  -- :: -- :: bK :: -- ::
        7  :: -- :: -- :: -- bp --
        6  -- :: -- :: -- :: -- ::
        5  :: -- :: -- :: -- :: --
        4  -- :: wp bp -- bB -- ::
        3  :: -- :: bp wK bp :: --
        2  -- :: -- bp -- bp -- ::
        1  :: -- :: -- :: -- :: --
            a  b  c  d  e  f  g  h
    ",
    )?;
    let game = Game::from_board(board, White);

    let expected_moves = [
        // Move::from_long_notation("e3d2"), // protected by bishop
        Move::from_long_notation("e3d3"),
        Move::from_long_notation("e3d4"),
        // Move::from_long_notation("e3e2"), // protected by pawns
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
               a  b  c  d  e  f  g  h",
    )?;
    let game = Game::from_board(board, White);
    let expected_move = Move::from_long_notation("e1c1");

    let moves_from_pos = game.get_moves_from_pos(Pos::from_notation("e1")?);

    assert!(
        moves_from_pos.contains(&expected_move),
        "{:?} didn't contain {}",
        moves_from_pos,
        expected_move
    );

    Ok(())
}

// TODO: Check for short castle when under attack, when spaces between are occupied
