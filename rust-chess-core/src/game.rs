use crate::board::PieceColor::{Black, White};
use crate::board::{Board, PieceColor, PieceType};
use crate::r#move::Move;
use crate::piece_moves_iterator::PieceMovesIter;
use crate::pos::Pos;
use regex::Regex;
use std::sync::LazyLock;

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
        let mut game = Game {
            history: GameHistory::new(),
            board: Board::new_chess_game(),
            possible_moves: Vec::new(),
            is_check: false,
            turn: White,
            result: None,
        };
        game.collect_possible_moves();
        game
    }

    pub fn from_board(board: Board, turn: PieceColor) -> Game {
        let mut game = Game {
            history: GameHistory::new(),
            board,
            possible_moves: Vec::new(),
            is_check: false,
            turn,
            result: None,
        };
        game.collect_game_state();
        game
    }

    pub fn from_board_with_history(board: Board, turn: PieceColor, history: GameHistory) -> Game {
        let mut game = Game {
            history,
            board,
            possible_moves: Vec::new(),
            is_check: false,
            turn,
            result: None,
        };
        game.collect_game_state();
        game
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

    pub fn is_check(&self) -> bool { self.is_check }

    pub fn make_move(&mut self, mv: &Move) -> Result<(), &'static str> {
        if self.result.is_some() {
            return Err("Game is over");
        }

        // Validate the move
        self.validate_move(&mv)?;

        // Update the board
        self.board.make_move(mv);

        // Update the history
        self.history.moves.push(*mv);

        // Switch turns
        self.turn = if self.turn == White { Black } else { White };

        // Check for game end conditions
        self.collect_game_state();

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

        if piece != PieceType::Pawn && mv.promotion_to.is_some() {
            return Err("Only pawns can be promoted");
        }

        let is_valid_move: bool = match piece {
            PieceType::Pawn => self.validate_pawn_move(mv, color).is_ok(),
            PieceType::Bishop => self.board.is_possible_bishop_capture(mv),
            PieceType::Knight => mv.is_knight_move(),
            PieceType::Rook => self.board.is_possible_rook_capture(mv),
            PieceType::Queen => self.board.is_possible_queen_capture(mv),
            PieceType::King => self.validate_king_move(mv).is_ok(),
        };

        if !is_valid_move {
            return Err("Invalid move for the piece");
        }

        let mut imitated_board = self.board.clone();
        imitated_board.make_move(&mv);
        if imitated_board.is_check(color) {
            return Err("King would be under attack");
        }

        Ok(())
    }

    pub fn parse_short_notation(&self, s: &str) -> Result<Move, String> {
        const SHORT_NOTATION_REGEX: &str = r"(?x)
            (?<piece>[RBNKQ])?
            (?<disambig_col>[a-h])?
            (?<disambig_row>[1-8])?
            (?<takes>x)?
            (?<col>[a-h])
            (?<row>[1-8])
            (?<promotion>(([=/])?([RBNQ]))|(\(([RBNQ])\)))?
            (?<appendix>[+\#])?
            ";
        static REGEX: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(SHORT_NOTATION_REGEX).expect("Invalid SHORT_NOTATION_REGEX")
        });

        // enum Appendix {
        //     None,
        //     Check,
        //     Mate,
        // }

        if s.starts_with("0-0-0") || s.starts_with("o-o-o") || s.starts_with("O-O-O") {
            // queenside castle
            let (row, col_from, col_to) = if self.turn == White {
                (0, 4, 2)
            } else {
                (7, 4, 2)
            };
            let mv = Move::new(col_from, row, col_to, row);
            if self.validate_move(&mv).is_err() {
                return Err("Invalid castle move".to_string());
            }
            return Ok(mv);
        } else if s.starts_with("0-0") || s.starts_with("o-o") || s.starts_with("O-O") {
            // kingside castle
            let (row, col_from, col_to) = if self.turn == White {
                (0, 4, 6)
            } else {
                (7, 4, 6)
            };
            let mv = Move::new(col_from, row, col_to, row);
            if self.validate_move(&mv).is_err() {
                return Err("Invalid castle move".to_string());
            }
            return Ok(mv);
        } 

        let Some(captures) = REGEX.captures(s) else {
            return Err("Failed to parse short notation string".to_string());
        };

        let piece = captures.name("piece");
        let disambig_col = captures.name("disambig_col");
        let disambig_row = captures.name("disambig_row");
        let takes = captures.name("takes");
        let col = &captures["col"];
        let row = &captures["row"];
        let promotion = captures.name("promotion");
        // let appendix = captures.name("appendix");

        let piece = if let Some(piece) = piece {
            PieceType::from_str(piece.as_str()).unwrap()
        } else {
            PieceType::Pawn
        };
        let disambig_col = if let Some(disambig_col) = disambig_col {
            Some((disambig_col.as_str().as_bytes()[0] - b'a') as i8)
        } else {
            None
        };
        let disambig_row = if let Some(disambig_row) = disambig_row {
            Some((disambig_row.as_str().as_bytes()[0] - b'1') as i8)
        } else {
            None
        };
        let takes = takes.is_some();
        let to_col = (col.as_bytes()[0] - b'a') as i8;
        let to_row = (row.as_bytes()[0] - b'1') as i8;
        let promotion = match promotion {
            None => None,
            Some(prom) => match prom.as_str().len() {
                1 => Some(PieceType::from_str(prom.as_str()).unwrap()),
                2 | 3 => Some(PieceType::from_char(prom.as_str().chars().nth(1).unwrap()).unwrap()),
                _ => panic!("Invalid promotion length"),
            },
        };

        // let appendix = match appendix {
        //     None => Appendix::None,
        //     Some(app) => {
        //         match app.as_str() {
        //             "+" => Appendix::Check,
        //             "#" => Appendix::Mate,
        //             _ => panic!("Invalid appendix"),
        //         }
        //     }
        // };

        let target_sq = self.board.at(to_col, to_row);
        let target_piece = target_sq.piece();
        let target_color = target_sq.piece_color();

        if target_piece.is_some() && target_color == self.turn {
            return Err("Cannot move to your own piece".to_string());
        }

        // Find from and to positions, validate move, check if it gives check/mate
        let mut result_mv: Option<Move> = None;
        let col_range = if let Some(dcol) = disambig_col {
            dcol..=dcol
        } else {
            0..=7
        };
        for col in col_range {
            let row_range = if let Some(d_row) = disambig_row {
                d_row..=d_row
            } else {
                0..=7
            };
            for row in row_range {
                let sq = self.board.at(col, row);
                let Some((piece_type, piece_color)) = sq.piece() else {
                    continue;
                };
                if piece_color != self.turn {
                    continue;
                };
                if piece_type != piece {
                    continue;
                };

                let mv = if let Some(prom) = promotion {
                    Move::with_promotion(col, row, to_col, to_row, prom)
                } else {
                    Move::new(col, row, to_col, to_row)
                };

                if takes && target_piece.is_none() && piece_type != PieceType::Pawn {
                    return Err("Invalid capture move".to_string());
                    // otherwise we validate en passant in validate_move
                } else if !takes && target_piece.is_some() {
                    return Err("Invalid non-capture move".to_string());
                }

                if self.validate_move(&mv).is_err() {
                    continue;
                }

                if takes && target_piece.is_none() && piece_type == PieceType::Pawn {
                    if self.board.is_en_passant_move(&mv).is_none() {
                        continue;
                    }
                }

                if result_mv != None {
                    return Err(String::from("Ambiguous move"));
                }
                result_mv = Some(mv);
            }
        }

        match result_mv {
            Some(mv) => Ok(mv),
            None => Err("No valid move found".to_string()),
        }
    }

    pub fn get_moves_from(&self, col: i8, row: i8) -> &[Move] {
        let mut from = 0;
        while from < self.possible_moves.len()
            && (self.possible_moves[from].from_col < col
                || self.possible_moves[from].from_row < row)
        {
            from += 1;
        }
        let mut to = from;
        while to < self.possible_moves.len()
            && self.possible_moves[to].from_col == col
            && self.possible_moves[to].from_row == row
        {
            to += 1;
        }
        &self.possible_moves[from..to]
    }

    pub fn get_moves_from_pos(&self, pos: Pos) -> &[Move] {
        self.get_moves_from(pos.col(), pos.row())
    }

    /// Collects possible moves and checks for check, checkmate, stalemate
    fn collect_game_state(&mut self) {
        self.is_check = self.board.is_check(self.turn);

        self.collect_possible_moves();
        if self.possible_moves.is_empty() {
            if self.is_check {
                self.result = Some(GameResult {
                    winner: Some(self.turn.opposite()),
                })
            } else {
                self.result = Some(GameResult { winner: None })
            }
        }
    }

    /// Returns true if it is a promotion
    fn validate_pawn_move(&self, mv: &Move, color: PieceColor) -> Result<bool, &'static str> {
        // if it's a move, not a capture
        if mv.is_pawn_move(color) {
            let target_sq = self.board.at(mv.to_col, mv.to_row);
            if target_sq.is_occupied() {
                return Err("Pawn move blocked by another piece");
            }
            if mv.from_row.abs_diff(mv.to_row) == 2 {
                let row = if color == White { mv.from_row + 1 } else { mv.from_row - 1};
                let between_sq = self.board.at(mv.from_col, row);
                if between_sq.is_occupied() {
                    return Err("Pawn move blocked by another piece");
                }
            }
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
        PieceMovesIter::new(&self, piece_type, piece_color, from_col, from_row)
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
    pub fn new() -> GameHistory {
        GameHistory {
            initial_state: None,
            initial_turn: None,
            moves: Vec::new(),
        }
    }

    pub fn with_moves(moves: Vec<Move>) -> GameHistory {
        GameHistory {
            initial_state: None,
            initial_turn: None,
            moves,
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
