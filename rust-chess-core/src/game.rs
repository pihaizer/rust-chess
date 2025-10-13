use crate::board::PieceColor::{Black, White};
use crate::board::{Board, PieceColor, PieceType};
use crate::piece_moves_iterator::PieceMovesIter;
use crate::r#move::Move;

pub struct Game {
    board: Board,
    turn: PieceColor,
    possible_moves: Vec<Move>,
    is_check: bool,
    history: GameHistory,
    result: Option<GameResult>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            history: GameHistory::new(),
            board: Board::new_chess_game(),
            possible_moves: Vec::new(),
            is_check: false,
            turn: White,
            result: None,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn history(&self) -> &GameHistory {
        &self.history
    }
    pub fn result(&self) -> &Option<GameResult> {
        &self.result
    }

    pub fn make_move(&mut self, mv: &Move) -> Result<(), &'static str> {
        if self.result.is_some() {
            return Err("Game is over");
        }

        let sq = self.board.at(mv.from_col, mv.from_row).clone();

        // Validate the move
        self.validate_move(&mv)?;

        let new_piece_type = if let Some(promotion) = mv.promotion_to {
            promotion
        } else {
            sq.piece_type().unwrap()
        };

        // Update the board
        self.board.make_move(mv);
        self.board.clear_square(mv.from_col, mv.from_row);
        self.board
            .set(mv.to_col, mv.to_row, new_piece_type, sq.piece_color());

        // Update the history
        self.history.moves.push(*mv);

        // Switch turns
        self.turn = if self.turn == White { Black } else { White };

        // Check for game end conditions
        self.is_check = self.board.is_check(self.turn);

        self.collect_possible_moves();
        if self.possible_moves.is_empty() {
            if self.is_check {
                self.result = Some(GameResult {winner: Some(self.turn.opposite())})
            } else {
                self.result = Some(GameResult {winner: None})
            }
        }

        Ok(())
    }

    pub fn validate_move(&self, mv: &Move) -> Result<(), &'static str> {
        if mv.from_col > 7 || mv.from_row > 7 || mv.to_col > 7 || mv.to_row > 7 {
            return Err("Out of bounds");
        }

        if mv.from_col == mv.to_col && mv.from_row == mv.to_row {
            return Err("Can't move to the same square");
        }

        let from_square = self.board.at(mv.from_col, mv.from_row);
        let piece = from_square.piece_type();
        let color = from_square.piece_color();

        let Some(piece) = piece else {
            return Err("No piece at the source square");
        };
        if color != self.turn {
            return Err("Cannot move the opponent's piece");
        }

        let target_square = self.board.at(mv.to_col, mv.to_row);

        if target_square.is_occupied_by_color(color) {
            return Err("Cannot move on your own piece");
        }

        let mut is_promotion = false;
        // let mut en_passant_captured_at: Option<Coords> = None;

        let is_valid_move: bool = match piece {
            PieceType::Pawn => {
                let promotion = self.validate_pawn_move(mv, color)?;
                is_promotion = promotion;
                true
            }
            PieceType::Bishop => self.board.is_possible_bishop_capture(mv),
            PieceType::Knight => mv.is_knight_move(),
            PieceType::Rook => self.board.is_possible_rook_capture(mv),
            PieceType::Queen => self.board.is_possible_queen_capture(mv),
            PieceType::King => self.validate_king_move(mv).is_ok(),
        };

        if !is_valid_move { return Err ("Invalid move for the piece") }

        // existence of promotion for pawn is validated in validate_pawn_move
        if !is_promotion && mv.promotion_to.is_some() {
            return Err("Cannot promote a non-pawn move");
        }

        let mut imitated_board = self.board.clone();
        imitated_board.make_move(&mv);
        if imitated_board.is_check(color) {
            return Err("King would be under attack");
        }

        Ok(())
    }

    pub fn parse_short_notation(&self, s: &str) -> Move {
        todo!()
    }

    /// Returns true if it is a promotion
    fn validate_pawn_move(
        &self,
        mv: &Move,
        color: PieceColor,
    ) -> Result<bool, &'static str> {
        // if it's a move, not a capture
        if mv.is_pawn_move(color) {
            if mv.to_row == 7 || mv.to_row == 0 {
                Ok(true)
            } else {
                Ok(false)
            }
        } else if mv.is_pawn_capture(color) {
            // can be either normal capture or en passant
            return if let Some(en_passant_pos) = self.board.is_en_passant_move(mv) {
                let en_passant_move_from: (i8, i8) = if color == White {
                    (mv.to_col, 6)
                } else {
                    (mv.to_col, 1)
                };
                let Some(last_move) = self.history.moves.last() else {
                    return Err("Invalid move");
                };
                if last_move.from() != en_passant_move_from
                    || last_move.to() != en_passant_pos.tuple()
                {
                    Err("Invalid move")
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            };
        } else {
            Err("Invalid move")
        }
    }

    fn validate_king_move(&self, mv: &Move) -> Result<(), &'static str> {
        if mv.is_regular_king_move() || self.is_legal_castle_move(mv) {
            Ok(())
        } else {
            Err("Invalid move")
        }
    }

    fn is_legal_castle_move(&self, mv: &Move) -> bool {
        let Some((old_rook_pos, _)) = self.board.is_possible_castle_move(mv) else {
            return false;
        };
        if self.is_check {
            return false;
        }

        // Validate that rook or king haven't moved
        let had_moves_from_rook_or_king = self.history.moves.iter().any(|h_mv| {
            // on the same row as king and either king col or rook col
            h_mv.from_row == mv.from_row
                && (h_mv.from_col == mv.from_col || h_mv.from_col == old_rook_pos.col())
        });
        if had_moves_from_rook_or_king {
            return false;
        }

        true
    }

    fn collect_possible_moves(&mut self) {
        let mut new_moves = Vec::with_capacity(20);
        for row in 0..8 {
            for col in 0..8 {
                if self.board.at(col, row).is_empty() {
                    continue;
                }
                for mv in self.iterate_through_moves(col, row) {
                    new_moves.push(mv);
                }
            }
        }
        self.possible_moves = new_moves;
    }

    fn iterate_through_moves(&self, from_col: i8, from_row: i8) -> impl Iterator<Item = Move> {
        let square = self.board.at(from_col, from_row);
        let piece_type = square.piece_type();
        let piece_color = square.piece_color();
        PieceMovesIter::new(&self, piece_type, piece_color, from_col, from_row, false)
    }
}

pub struct GameResult {
    // None in case of a draw
    pub winner: Option<PieceColor>,
}

pub struct GameHistory {
    // Only set if the initial state is not the standard chess starting position
    initial_state: Option<Board>,
    initial_turn: Option<PieceColor>,
    moves: Vec<Move>,
}

impl GameHistory {
    fn new() -> GameHistory {
        GameHistory {
            initial_state: None,
            initial_turn: None,
            moves: Vec::new(),
        }
    }

    pub fn moves(&self) -> &Vec<Move> {
        &self.moves
    }

    pub fn initial_state(&self) -> &Option<Board> {
        &self.initial_state
    }

    pub fn initial_turn(&self) -> &Option<PieceColor> {
        &self.initial_turn
    }
}
