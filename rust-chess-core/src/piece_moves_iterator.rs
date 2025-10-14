use crate::board::PieceColor::White;
use crate::board::{PieceColor, PieceType, KING_OFFSETS, KNIGHT_OFFSETS};
use crate::game::Game;
use crate::r#move::{Move};
use crate::pos::Pos;

// we want to have one iterator for all pieces so that we can use it without dyn and therefore without heap allocation
pub struct PieceMovesIter<'a> {
    game: &'a Game,
    piece_type: Option<PieceType>,
    piece_color: PieceColor,
    from: Pos,
    current: Pos,

    // different for different pieces.
    // pawn: 0 = straight, 1 = diagonal
    // rook: 0 = horizontal right, 1 = horizontal left, 2 = vertical up, 3 = vertical down
    // bishop: 0 = diagonal up-right, 1 = diagonal up-left, 2 = diagonal down-right, 3 = diagonal down-left
    // knight: 0..7 = all 8 possible moves
    // queen: same as rook and bishop combined,
    // king: 0..7 = all 8 possible moves
    phase: i8,
}

impl<'a> Iterator for PieceMovesIter<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(piece_type) = self.piece_type else {
            return None;
        };
        loop {
            let mv = match piece_type {
                PieceType::Pawn => self.next_pawn(),
                PieceType::Bishop => self.next_bishop(),
                PieceType::Knight => self.next_knight(),
                PieceType::Rook => self.next_rook(),
                PieceType::Queen => self.next_queen(),
                PieceType::King => self.next_king(),
            };
            let Some(mv) = mv else { return None };
            if self.game.validate_move(&mv).is_ok() {
                return Some(mv)
            }
        }
    }
}

impl<'a> PieceMovesIter<'a> {
    pub fn new(
        game: &'a Game,
        piece_type: Option<PieceType>,
        piece_color: PieceColor,
        from_col: i8,
        from_row: i8,
    ) -> PieceMovesIter<'a> {
        PieceMovesIter {
            game,
            piece_type,
            piece_color,
            from: Pos::new(from_col, from_row),
            current: Pos::new(from_col, from_row),
            phase: 0,
        }
    }

    fn next_pawn(&mut self) -> Option<Move> {
        let row = if self.piece_color == White {
            self.from.row() + 1
        } else {
            self.from.row() - 1
        };

        loop {
            let pos = match self.phase {
                0 => {
                    // straight move
                    let col = self.from.col();
                    Pos::new(col, row)
                }
                1 => {
                    // straight move 2 up
                    let row = if self.piece_color == White {
                        self.from.row() + 2
                    } else {
                        self.from.row() - 2
                    };
                    Pos::new(self.from.col(), row)
                }
                2 | 3 => {
                    // diagonal left
                    let col = self.from.col() + if self.phase == 2 { 1 } else { -1 };
                    Pos::new(col, row)
                }
                4 => {
                    self.set_next_phase();
                    return None
                },
                _ => panic!("Pawn iterator phase out of bounds"),
            };
            if pos.is_out_of_bounds() {
                self.set_next_phase();
                continue;
            }
            self.phase += 1;
            let is_promotion = pos.row() == 0 || pos.row() == 7;
            return if is_promotion {
                Some(Move::with_promotion_from_pos(&self.from, &pos, PieceType::Queen))
            } else {
                self.move_to(&pos)
            }
        }
    }

    fn next_rook(&mut self) -> Option<Move> {
        loop {
            if self.increment_rook(self.phase).is_some() {
                let res = self.next_simple();
                if let Some(mv) = res {
                    return Some(mv);
                } else {
                    continue;
                }
            } else {
                return None
            }
        }
    }

    fn next_bishop(&mut self) -> Option<Move> {
        loop {
            if self.increment_bishop(self.phase).is_some() {
                let res = self.next_simple();
                if let Some(mv) = res {
                    return Some(mv);
                } else {
                    continue;
                }
            } else {
                return None
            }
        }
    }

    fn next_knight(&mut self) -> Option<Move> {
        loop {
            if self.phase >= 9 {
                panic!("Knight iterator phase out of bounds");
            } else if self.phase == 8 {
                self.phase += 1;
                return None;
            }

            let to = self.from + KNIGHT_OFFSETS[self.phase as usize];
            self.set_next_phase();
            if to.is_out_of_bounds() {
                continue;
            }
            return self.move_to(&to);
        }
    }

    fn next_queen(&mut self) -> Option<Move> {
        loop {
            let incr_result: Option<()>;
            if self.phase < 4 {
                incr_result = self.increment_rook(self.phase)
            } else {
                incr_result = self.increment_bishop(self.phase - 4)
            }
            if incr_result.is_some() {
                let res = self.next_simple();
                if let Some(mv) = res {
                    return Some(mv);
                } else {
                    continue;
                }
            } else {
                return None
            }
        }
    }

    fn next_king(&mut self) -> Option<Move> {
        loop {
            match self.phase {
                0..=7 => {
                    let to = self.from + KING_OFFSETS[self.phase as usize];
                    self.set_next_phase();
                    if to.is_out_of_bounds() {
                        continue;
                    }
                    return self.move_to(&to);
                }
                8..=9 => {
                    // castle moves
                    let castle_row: i8 = if self.piece_color == White { 0 } else { 7 };
                    if self.from.row() != castle_row || self.from.col() != 4 {
                        assert_eq!(self.phase, 8);
                        self.phase += 2; // skip both castles. Should not be reachable on phase 9
                        continue;
                    }
                    let to_col = if self.phase == 8 { 6 } else { 2 };
                    let to = Pos::new(to_col, castle_row);
                    self.phase += 1;
                    return self.move_to(&to);
                }
                10 => {
                    self.phase += 1;
                    return None;
                }
                _ => {
                    panic!("Knight iterator phase out of bounds");
                }
            }
        }
    }

    fn next_simple(&mut self) -> Option<Move> {
        loop {
            if self.current.is_out_of_bounds() {
                self.set_next_phase();
                return None;
            }
            let target = self.game.board().at(self.current.col(), self.current.row());
            if target.is_empty() {
                return self.move_to(&self.current);
            } else if target.is_occupied_by_color(self.piece_color.opposite()) {
                let current = &self.current.clone();
                self.set_next_phase();
                return self.move_to(current);
            } else {
                self.set_next_phase();
                return None;
            }
        }
    }

    // fn check_en_passant(&self, to_col: i8) -> Result<(i8, i8), ()> {
    //     let en_passant_move_from: (i8, i8);
    //     let en_passant_captured_coords = (to_col, self.from_row);
    //     let en_passant_square = self
    //         .game
    //         .board()
    //         .at(en_passant_captured_coords.0, en_passant_captured_coords.1);
    //
    //     if en_passant_square.is_empty() {
    //         return Err(());
    //     }
    //     if en_passant_square.piece_type().unwrap() != PieceType::Pawn {
    //         return Err(());
    //     }
    //     if self.piece_color == White {
    //         if self.from_row != 4 {
    //             return Err(());
    //         }
    //         en_passant_move_from = (to_col, 6)
    //     } else {
    //         if self.from_row != 3 {
    //             return Err(());
    //         }
    //         en_passant_move_from = (to_col, 1)
    //     }
    //     let Some(last_move) = self.game.history().moves().last() else {
    //         return Err(());
    //     };
    //     if last_move.from() != en_passant_move_from || last_move.to() != en_passant_captured_coords
    //     {
    //         return Err(());
    //     }
    //
    //     Ok(en_passant_captured_coords)
    // }

    const ROOK_INCREMENTS: [Pos; 4] = [
        Pos::new(1, 0),
        Pos::new(-1, 0),
        Pos::new(0, 1),
        Pos::new(0, -1),
    ];

    const BISHOP_INCREMENTS: [Pos; 4] = [
        Pos::new(1, 1),
        Pos::new(1, -1),
        Pos::new(-1, 1),
        Pos::new(-1, -1),
    ];

    fn increment_rook(&mut self, phase: i8) -> Option<()> {
        match phase {
            0..=3 => {
                self.current += Self::ROOK_INCREMENTS[phase as usize];
                Some(())
            }
            4 => {
                self.phase += 1;
                None
            }
            _ => panic!("Rook iterator phase out of bounds"),
        }
    }

    fn increment_bishop(&mut self, phase: i8) -> Option<()> {
        match phase {
            0..=3 => {
                self.current += Self::BISHOP_INCREMENTS[phase as usize];
                Some(())
            }
            4 => {
                self.phase += 1;
                None
            }
            _ => panic!("Bishop iterator phase out of bounds"),
        }
    }

    fn set_next_phase(&mut self) {
        self.phase += 1;
        self.current = self.from;
    }

    fn move_to(&self, to: &Pos) -> Option<Move> { Some(Move::from_pos(&self.from, to)) }
}
