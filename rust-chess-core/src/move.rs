use std::fmt::{Debug, Display};
use crate::board::PieceColor::White;
use crate::board::{Board, PieceColor, PieceType};
use crate::pos::Pos;

#[derive(Clone, Copy, PartialEq)]
pub struct Move {
    pub from_col: i8,
    pub from_row: i8,
    pub to_col: i8,
    pub to_row: i8,
    pub promotion_to: Option<PieceType>,
}

impl Move {
    pub fn new(from_col: i8, from_row: i8, to_col: i8, to_row: i8) -> Move {
        Board::panic_if_out_of_bounds(from_col, from_row);
        Board::panic_if_out_of_bounds(to_col, to_row);

        Move {
            from_col,
            from_row,
            to_col,
            to_row,
            promotion_to: None,
        }
    }

    pub fn from_pos(from: &Pos, to: &Pos) -> Move {
        Move::new(from.col(), from.row(), to.col(), to.row())
    }

    pub fn with_promotion(
        from_col: i8,
        from_row: i8,
        to_col: i8,
        to_row: i8,
        promotion_to: PieceType,
    ) -> Move {
        Board::panic_if_out_of_bounds(from_col, from_row);
        Board::panic_if_out_of_bounds(to_col, to_row);
        if !promotion_to.is_valid_for_promotion() {
            panic!("Invalid promotion piece type");
        }

        Move {
            from_col,
            from_row,
            to_col,
            to_row,
            promotion_to: Some(promotion_to),
        }
    }

    pub fn with_promotion_from_pos(
        from: &Pos,
        to: &Pos,
        promotion_to: PieceType,
    ) -> Move {
        Move::with_promotion(from.col(), from.row(), to.col(), to.row(), promotion_to)
    }

    pub fn from_long_notation(s: &str) -> Move {
        if s.len() < 4 {
            panic!("Invalid move notation");
        }

        let from_col = s.chars().nth(0).unwrap() as i8 - 'a' as i8;
        let from_row = s.chars().nth(1).unwrap() as i8 - '1' as i8;
        let to_col = s.chars().nth(2).unwrap() as i8 - 'a' as i8;
        let to_row = s.chars().nth(3).unwrap() as i8 - '1' as i8;

        if s.len() == 5 {
            let promotion_char = s.chars().nth(4).unwrap();
            let promotion_to = match promotion_char {
                'q' | 'Q' => PieceType::Queen,
                'r' | 'R' => PieceType::Rook,
                'b' | 'B' => PieceType::Bishop,
                'n' | 'N' => PieceType::Knight,
                _ => panic!("Invalid promotion piece type"),
            };
            Move::with_promotion(from_col, from_row, to_col, to_row, promotion_to)
        } else {
            Move::new(from_col, from_row, to_col, to_row)
        }
    }

    pub fn from(&self) -> (i8, i8) {
        (self.from_col, self.from_row)
    }

    pub fn to(&self) -> (i8, i8) {
        (self.to_col, self.to_row)
    }

    pub fn has_promotion(&self) -> bool {
        self.promotion_to.is_some()
    }

    pub fn is_diagonal(&self) -> bool {
        i8::abs_diff(self.from_col, self.to_col) == i8::abs_diff(self.from_row, self.to_row)
    }

    pub fn is_straight(&self) -> bool {
        self.from_col == self.to_col || self.from_row == self.to_row
    }

    pub fn is_pawn_move(&self, pawn_color: PieceColor) -> bool {
        if self.from_col != self.to_col { return false }
        let valid_pawn_direction : i8;
        let row_for_two_step: i8;
        if pawn_color == White {
            valid_pawn_direction = 1;
            row_for_two_step = 1;
        } else {
            valid_pawn_direction = -1;
            row_for_two_step = 6;
        }
        let is_step_ok = (self.to_row == self.from_row + valid_pawn_direction) ||
            (self.to_row == self.from_row + valid_pawn_direction * 2 && self.from_row == row_for_two_step);
        if !is_step_ok { return false }
        if (self.to_row == 0 || self.to_row == 7) && self.promotion_to.is_none() {
            return false;
        } else if self.to_row >= 1 && self.to_row <= 6 && self.promotion_to.is_some() {
            return false;
        }
        true
    }

    /// Checks if the given move is a pawn capture move for the given pawn color
    pub fn is_pawn_capture(&self, pawn_color: PieceColor) -> bool {
        if pawn_color == White {
            if self.to_row != self.from_row + 1 {
                return false;
            }
        } else {
            if self.to_row != self.from_row - 1 {
                return false;
            }
        }
        if (self.from_col - self.to_col).abs() != 1 {
            return false;
        }
        true
    }

    pub fn is_knight_move(&self) -> bool {
        let vertical_move = i8::abs_diff(self.from_row, self.to_row);
        let horizontal_move = i8::abs_diff(self.from_col, self.to_col);
        vertical_move > 0 && horizontal_move > 0 && vertical_move + horizontal_move == 3
    }

    pub fn is_regular_king_move(&self) -> bool {
        let vertical_move = i8::abs_diff(self.from_row, self.to_row);
        let horizontal_move = i8::abs_diff(self.from_col, self.to_col);
        vertical_move <= 1 && horizontal_move <= 1
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let from_col = (self.from_col + 'a' as i8) as u8 as char;
        let from_row = (self.from_row + '1' as i8) as u8 as char;
        let to_col = (self.to_col + 'a' as i8) as u8 as char;
        let to_row = (self.to_row + '1' as i8) as u8 as char;
        if let Some(promotion_to) = self.promotion_to {
            let promotion_char = match promotion_to {
                PieceType::Queen => 'q',
                PieceType::Rook => 'r',
                PieceType::Bishop => 'b',
                PieceType::Knight => 'n',
                _ => '?',
            };
            write!(f, "{}{}{}{}{}", from_col, from_row, to_col, to_row, promotion_char)
        } else {
            write!(f, "{}{}{}{}", from_col, from_row, to_col, to_row)
        }
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}