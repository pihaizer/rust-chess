use rust_chess_core::board::{Board, PieceColor};
use rust_chess_core::game::Game;

#[test]
fn no_checkmate_if_can_take_attacking_piece() -> Result<(), String> {
    const BOARD_STR: &str = r"
8  -- :: -- bQ -- :: bK ::
7  bR -- bp bB :: -- bp --
6  bp :: -- :: -- bp wN bp
5  :: bp :: wp :: -- :: --
4  -- :: -- :: -- :: -- ::
3  :: wp wp -- :: -- :: --
2  wp :: wQ :: -- wp wp wp
1  wR -- :: -- bR -- wK --
    a  b  c  d  e  f  g  h";
    let board = Board::from_string(BOARD_STR)?;
    let mut game = Game::from_board(board, PieceColor::White);
    
    assert!(game.is_check());
    assert!(game.result().is_none());
    let mv = game.parse_short_notation("Rxe1")?;
    game.make_move(&mv)?;
    
    Ok(())
}