use crate::board::PieceColor::*;
use crate::board::PieceType::*;
use crate::r#move::{Move};
use std::fmt::{Debug, Display, Formatter};
use crate::pos::Pos;

#[derive(Copy, Clone, PartialEq)]
pub struct Board {
    // squares are stored line-by-line, starting with a1-h1, a2-h2, ..., a8-h8
    squares: [BoardSquare; 64],
}


const SYMBOLS_ROW: &str = "    a  b  c  d  e  f  g  h\n";

impl Board {
    pub fn empty() -> Board {
        Board {
            squares: [BoardSquare::empty(); 64],
        }
    }

    pub fn new_chess_game() -> Board {
        let mut board = Board::empty();

        board.set(0, 0, Rook, White);
        board.set(1, 0, Knight, White);
        board.set(2, 0, Bishop, White);
        board.set(3, 0, Queen, White);
        board.set(4, 0, King, White);
        board.set(5, 0, Bishop, White);
        board.set(6, 0, Knight, White);
        board.set(7, 0, Rook, White);

        for i in 0..8 {
            board.set(i, 1, Pawn, White)
        }

        for i in 0..8 {
            board.set(i, 6, Pawn, Black);
        }

        board.set(0, 7, Rook, Black);
        board.set(1, 7, Knight, Black);
        board.set(2, 7, Bishop, Black);
        board.set(3, 7, Queen, Black);
        board.set(4, 7, King, Black);
        board.set(5, 7, Bishop, Black);
        board.set(6, 7, Knight, Black);
        board.set(7, 7, Rook, Black);

        board
    }

    /// Parses a board state from string in the following format:
    /// ```
    /// -- :: -- :: bK :: -- ::
    /// :: -- :: -- bR -- :: --
    /// -- :: -- :: -- :: -- ::
    /// :: -- :: -- :: -- :: --
    /// -- :: -- :: -- :: -- ::
    /// :: -- :: -- :: -- :: --
    /// -- :: -- :: wp :: -- ::
    /// :: -- :: -- wK -- :: --
    /// ```
    /// Each row can optionally contain a row number at the start of the line and a column letters row at the end.
    /// Each square is represented by two characters: piece color (w or b) and piece type (p, R, N, B, Q, K).
    /// Empty squares are represented by "--" for white squares or "::" for black squares.
    /// Rows are separated by newlines. The first row corresponds to row 8, the last row to row 1.
    /// Columns are from 'a' to 'h', left to right.
    pub fn from_string(input: &str) -> Result<Board, String> {
        let mut board = Board::empty();
        for (row, line) in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).enumerate() {
            if row >= 9 {
                return Err(String::from("Too many lines in board string"));
            }
            if row == 8 { continue } // don't validate the last line if it's column letters
            let row = 7 - row;

            let squares: Vec<_> = line.split_whitespace().enumerate().collect();
            if squares.len() > 9 {
                return Err(String::from("Too many squares in a line"));
            }
            let has_row_number = squares.len() == 9;

            for (col, square) in squares {
                if has_row_number && col == 0 { continue };
                let col = if has_row_number { col - 1} else { col };
                let pos = Pos::new(col as i8, row as i8);
                match square {
                    "::" => {
                        if (col + row) % 2 != 0 {
                            return Err(format!("Invalid empty black square position at {pos}"));
                        }
                    }
                    "--" => {
                        if (col + row) % 2 != 1 {
                            return Err(format!("Invalid empty white square position at {pos}"));
                        }
                    }
                    s if s.len() == 2 => {
                        let color = match s.chars().nth(0).unwrap() {
                            'w' => White,
                            'b' => Black,
                            _other => return Err(format!("Invalid piece color {_other} at {pos}")),
                        };
                        let piece = match s.chars().nth(1).unwrap() {
                            'p' => Pawn,
                            'R' => Rook,
                            'N' => Knight,
                            'B' => Bishop,
                            'Q' => Queen,
                            'K' => King,
                            _other => return Err(format!("Invalid piece type {_other} at {pos}")),
                        };
                        board.set(col as i8, row as i8, piece, color)
                    }
                    _other => return Err(format!("Invalid square format {_other} at {pos}")),
                }
            }
        }

        Ok(board)
    }

    /// Makes move for pieces. Move is not validated here. En passant and castling are checked automatically.
    pub fn make_move(&mut self, mv: &Move) {
        let sq = self.at(mv.from_col, mv.from_row).clone();
        let piece_type = if let Some(promotion) = mv.promotion_to {
            promotion
        } else {
            sq.piece_type().unwrap()
        };

        self.clear_square(mv.from_col, mv.from_row);
        self.set(mv.to_col, mv.to_row, piece_type, sq.piece_color());

        if let Some(en_passant_at) = self.is_en_passant_move(mv) {
            self.clear_square(en_passant_at.col(), en_passant_at.row());
        }

        if let Some((old_rook_coords, new_rook_coords)) = self.is_castle_move(mv) {
            self.clear_square(old_rook_coords.col(), old_rook_coords.row());
            self.set(
                new_rook_coords.col(),
                new_rook_coords.row(),
                Rook,
                sq.piece_color(),
            );
        }
    }

    pub fn set(&mut self, col: i8, row: i8, piece: PieceType, color: PieceColor) {
        *self.at_mut(col, row) = BoardSquare::with(piece, color);
    }

    pub fn set_at_pos(&mut self, pos: &Pos, piece: PieceType, color: PieceColor) {
        self.set(pos.col(), pos.row(), piece, color);
    }

    pub fn clear_square(&mut self, col: i8, row: i8) {
        *self.at_mut(col, row) = BoardSquare::empty();
    }


    /// The board format is the following:
    /// ```
    /// 8  -- :: -- bK -- :: -- ::
    /// 7  :: -- :: -- bR -- :: --
    /// 6  -- :: -- :: -- :: -- ::
    /// 5  :: -- :: -- :: -- :: --
    /// 4  -- :: -- :: -- :: -- ::
    /// 3  :: -- :: -- :: -- :: --
    /// 2  -- :: -- :: wP :: -- ::
    /// 1  :: -- :: wK :: -- :: --
    ///     a  b  c  d  e  f  g  h
    /// ```
    /// If print_col_row_helpers is true, the row numbers and column letters are printed.
    pub fn get_display_str(&self, print_col_row_helpers: bool) -> String {
        // When printing row and col helpers, we have 9 rows.
        // Each row contains 2 symbols for number ("1 "), 3 symbols per each square * 8 (" bK"),
        // and closing '\n', resulting in 2 + 3 * 8 + 1 = 27. We also have '\n' at the start of the board.
        const FULL_BOARD_STRING_CAPACITY: usize = 1 + (2 + 3 * 8 + 1) * 9;

        // When printing without row and col helpers, we have 8 rows.
        // Each row contains 3 symbols per each square * 8 (" bK"), excluding the first square without a whitespace,
        // which is compensated by a closing '\n', resulting in 3 * 8 = 24. We also have '\n' at the start of the board.
        const SHORT_BOARD_STRING_CAPACITY: usize = 1 + (3 * 8) * 8;

        let mut result = String::with_capacity(if print_col_row_helpers { FULL_BOARD_STRING_CAPACITY } else { SHORT_BOARD_STRING_CAPACITY });
        result.push('\n');

        // print the board row-by-row starting from the topmost row which is 8
        for row in (0..8).rev() {
            if print_col_row_helpers {
                result.push_str((row + 1).to_string().as_str());
                result.push(' ');
            }

            for col in 0..8 {
                result.push(' ');

                let square = self.at(col, row);
                if square.is_occupied() {
                    result.push_str(square.to_string().as_str())
                } else {
                    result.push_str(if (row + col) % 2 == 0 { "::" } else { "--" })
                }
            }
            result.push_str("\n");
        }

        if print_col_row_helpers {
            result.push_str(SYMBOLS_ROW);
        }

        result
    }

    pub fn print(&self, print_col_row_helpers: bool) {
        println!("{}", self.get_display_str(print_col_row_helpers));
    }

    pub fn at(&self, col: i8, row: i8) -> &BoardSquare {
        &self.squares[Self::get_index(col, row)]
    }

    pub fn at_mut(&mut self, col: i8, row: i8) -> &mut BoardSquare {
        &mut self.squares[Self::get_index(col, row)]
    }

    pub fn panic_if_out_of_bounds(col: i8, row: i8) {
        if col < 0 || row < 0 || col > 7 || row > 7 {
            panic!("Column and row must be between 0 and 7 (inclusive)");
        }
    }

    fn get_index(col: i8, row: i8) -> usize {
        (row * 8 + col) as usize
    }

    /// Returns true if the move is a possible rook capture move. Doesn't validate the move in
    /// terms of colors, checks, etc., because Board doesn't have all info for that.
    pub fn is_possible_rook_capture(&self, mv: &Move) -> bool {
        mv.is_straight() && !self.is_move_over_pieces_straight(mv)
    }

    pub fn is_possible_bishop_capture(&self, mv: &Move) -> bool {
        mv.is_diagonal() && !self.is_move_over_pieces_diagonal(mv)
    }

    pub fn is_possible_queen_capture(&self, mv: &Move) -> bool {
        self.is_possible_rook_capture(mv) || self.is_possible_bishop_capture(mv)
    }

    /// Returns true if there is a piece on the way
    /// (from_col, from_row) and (to_col, to_row) must be in a straight line
    pub fn is_move_over_pieces_straight(&self, mv: &Move) -> bool {
        assert!(mv.is_straight());
        let is_moving_horizontally = mv.from_row == mv.to_row;

        // fail if there are any pieces on the way to target square
        if is_moving_horizontally {
            let from = i8::min(mv.from_col, mv.to_col) + 1;
            let to = i8::max(mv.from_col, mv.to_col);
            for col in from..to {
                let square = self.at(col, mv.from_row);
                if square.is_occupied() {
                    return true;
                }
            }
        } else {
            let from = i8::min(mv.from_row, mv.to_row) + 1;
            let to = i8::max(mv.from_row, mv.to_row);
            for row in from..to {
                let square = self.at(mv.from_col, row);
                if square.is_occupied() {
                    return true;
                }
            }
        }

        false
    }

    fn is_move_over_pieces_diagonal(&self, mv: &Move) -> bool {
        assert!(mv.is_diagonal());
        // let move_horizontal: i8 = if mv.to_col > mv.from_col { 1 } else { -1 };
        let move_horizontal = (mv.to_col - mv.from_col).signum();
        // let move_vertical: i8 = if mv.to_row > mv.from_row { 1 } else { -1 };
        let move_vertical = (mv.to_row - mv.from_row).signum();

        // fail if there are any pieces on the way to target square
        for i in 1..(i8::abs_diff(mv.from_col, mv.to_col) as i8) {
            let col = mv.from_col + i * move_horizontal;
            let row = mv.from_row + i * move_vertical;

            let square = self.at(col, row);
            if square.is_occupied() {
                return true;
            }
        }
        false
    }

    /// Checks if move is en-passant and returns captured coordinates if yes
    pub fn is_en_passant_move(&self, mv: &Move) -> Option<Pos> {
        let sq = self.at(mv.from_col, mv.from_row);
        if sq.piece_type() != Some(Pawn) {
            return None;
        }
        if !mv.is_pawn_capture(sq.piece_color) {
            return None;
        }
        let captured_sq = self.at(mv.to_col, mv.to_row);
        if captured_sq.is_occupied() {
            return None;
        }
        Some(Pos::new(mv.from_col, mv.to_row))
    }

    /// Checks if the move itself is from and to the right squares to be a castle move,
    /// Doesn't check if there is a rook, the king or rook have moved before, or if the king is in check or passes through check
    /// Returns coords of rook before and after castling if it is a castle move, None otherwise
    pub fn is_castle_move(&self, mv: &Move) -> Option<(Pos, Pos)> {
        if (mv.from_row != 0 && mv.from_row != 7) || mv.from_col != 4 {
            return None;
        }
        if mv.to_row != mv.from_row {
            return None;
        }
        let square = self.at(mv.from_col, mv.from_row);
        let Some((piece_type, piece_color)) = square.piece() else {
            return None;
        };
        if piece_type != King {
            return None;
        }
        if piece_color == White && mv.from_row != 0 {
            return None;
        } else if piece_color == Black && mv.from_row != 7 {
            return None;
        }
        if mv.to_col != 6 && mv.to_col != 2 {
            return None;
        }

        let rook_col = if mv.to_col == 6 { 7 } else { 0 };
        let new_rook_col = if mv.to_col == 6 { 5 } else { 3 };
        Some((
            Pos::new(rook_col, mv.from_row),
            Pos::new(new_rook_col, mv.from_row),
        ))
    }

    /// Checks everything "is_castle_move" does, and also checks if there is no pieces between king and rook,
    /// if the rook and king are on their original squares, and if the king is not in check or passes through check.
    ///
    /// Returns Some((rook_old_coords, rook_new_coords)) if the move is a possible castle move, None otherwise.
    ///
    /// Note: this function can't check if the king or rook have moved before, because Board doesn't have that info
    pub fn is_possible_castle_move(&self, mv: &Move) -> Option<(Pos, Pos)> {
        let Some((rook_pos, new_rook_pos)) = self.is_castle_move(mv) else {
            return None;
        };
        let king_color = self.at(mv.from_col, mv.from_row).piece_color();

        let Some((piece_type, piece_color)) = self.at(rook_pos.col(), rook_pos.row()).piece()
        else {
            return None;
        };
        if piece_type != Rook || piece_color != king_color {
            return None;
        }

        let under_attack_check_range = if mv.to_col == 6 { 4..=6 } else { 2..=4 };
        let occupied_check_range = if mv.to_col == 6 { 5..=6 } else { 1..=3 };
        for col in under_attack_check_range {
            if self.is_under_attack(col, mv.from_row, king_color.opposite())
            {
                return None;
            }
        }
        for col in occupied_check_range {
            if self.at(col, mv.from_row).is_occupied()
            {
                return None;
            }
        }

        Some((rook_pos, new_rook_pos))
    }

    pub fn find_king(&self, king_color: PieceColor) -> Option<Pos> {
        let mut king_col: i8 = -1;
        let mut king_row: i8 = -1;
        // find the king
        'outer: for col in 0..8 {
            for row in 0..8 {
                let Some((piece, color)) = self.at(col, row).piece() else {
                    continue;
                };
                if color == king_color && piece == King {
                    king_col = col;
                    king_row = row;
                    break 'outer;
                }
            }
        }
        if king_col >= 0 {
            Some(Pos::new(king_col, king_row))
        } else {
            None
        }
    }

    pub fn is_under_attack(
        &self,
        target_col: i8,
        target_row: i8,
        attacking_color: PieceColor,
    ) -> bool {
        for col in 0..8 {
            for row in 0..8 {
                let Some((piece, color)) = self.at(col, row).piece() else {
                    continue;
                };
                if color != attacking_color {
                    continue;
                }
                let mv = Move::new(col, row, target_col, target_row);

                let is_valid_move = match piece {
                    Pawn => mv.is_pawn_capture(color),
                    Bishop => self.is_possible_bishop_capture(&mv),
                    Knight => mv.is_knight_move(),
                    Rook => self.is_possible_rook_capture(&mv),
                    Queen => self.is_possible_queen_capture(&mv),
                    King => mv.is_regular_king_move(),
                };
                if is_valid_move {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_check(&self, king_color: PieceColor) -> bool {
        let king = self.find_king(king_color).expect("No king on the board!");
        self.is_under_attack(king.col(), king.row(), king_color.opposite())
    }

    // We can check for check, but not for mate or stalemate.
    // There can be a situation where en passant is the only legal move under check,
    // e.g. "k7/5p2/4p3/6P1/6K1/r7/7q/8 b - - 0 1" after f5 -> therefore checking mate is not possible
    // "k7/5p2/4p1p1/6P1/5K2/8/6q1/4r3 b - - 0 2" - analogous position for stalemate.
    // after f5 only legal move is en passant, but without en passant it's stalemate.
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_display_str(true))
    }
}

pub const KING_OFFSETS: [Pos; 8] = [
    Pos::new(-1, -1),
    Pos::new(0, -1),
    Pos::new(1, -1),
    Pos::new(-1, 0),
    Pos::new(1, 0),
    Pos::new(-1, 1),
    Pos::new(0, 1),
    Pos::new(1, 1),
];

pub const KNIGHT_OFFSETS: [Pos; 8] = [
    Pos::new(2, 1),
    Pos::new(2, -1),
    Pos::new(-2, 1),
    Pos::new(-2, -1),
    Pos::new(1, 2),
    Pos::new(1, -2),
    Pos::new(-1, 2),
    Pos::new(-1, -2),
];

#[derive(PartialEq, Copy, Clone)]
pub struct BoardSquare {
    piece_type: Option<PieceType>,
    piece_color: PieceColor,
}

impl BoardSquare {
    pub fn piece_type(&self) -> Option<PieceType> {
        self.piece_type
    }

    pub fn piece_color(&self) -> PieceColor {
        self.piece_color
    }

    pub fn empty() -> BoardSquare {
        BoardSquare {
            piece_type: None,
            piece_color: White,
        }
    }

    pub fn with(piece_type: PieceType, piece_color: PieceColor) -> BoardSquare {
        BoardSquare {
            piece_type: Some(piece_type),
            piece_color,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.piece_type == None
    }
    pub fn is_occupied(&self) -> bool {
        !self.is_empty()
    }
    pub fn is_occupied_by_color(&self, color: PieceColor) -> bool {
        self.is_occupied() && self.piece_color == color
    }
    pub fn piece(&self) -> Option<(PieceType, PieceColor)> {
        match self.piece_type {
            Some(piece_type) => Some((piece_type, self.piece_color)),
            None => None,
        }
    }
}

impl Display for BoardSquare {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(piece_type) = self.piece_type {
            write!(f, "{}{}", self.piece_color, piece_type)
        } else {
            write!(f, "..")
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn is_valid_for_promotion(&self) -> bool {
        match self {
            Pawn | King => false,
            Bishop | Knight | Rook | Queen => true,
        }
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Pawn => "p",
            Bishop => "B",
            Knight => "N",
            Rook => "R",
            Queen => "Q",
            King => "K",
        };
        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn opposite(&self) -> PieceColor {
        match self {
            White => Black,
            Black => White,
        }
    }
}

impl Display for PieceColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            White => "w",
            Black => "b",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_board_str_works() {
        let board_parsed = Board::from_string("
            -- :: -- bK -- :: -- ::
            :: -- :: -- bR -- :: --
            -- :: -- :: -- :: -- ::
            :: -- :: -- :: -- :: --
            -- :: -- :: -- :: -- ::
            :: -- :: -- :: -- :: --
            -- :: -- :: wp :: -- ::
            :: -- :: wK :: -- :: --
        ").expect("Failed to parse board string");

        let mut board_manual = Board::empty();
        board_manual.set(3, 0, King, White);
        board_manual.set(4, 1, Pawn, White);
        board_manual.set(4, 6, Rook, Black);
        board_manual.set(3, 7, King, Black);

        assert_eq!(board_parsed, board_manual);
    }
}