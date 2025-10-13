use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, PartialEq)]
pub struct Pos {
    col: i8,
    row: i8,
}

impl Pos {
    pub const fn new(col: i8, row: i8) -> Pos {
        Pos { col, row }
    }
    pub const fn from_notation(input: &str) -> Result<Pos, &'static str> {
        if input.len() != 2 {
            return Err("Invalid position notation length");
        }
        let bytes = input.as_bytes();
        let col = bytes[0];
        let row = bytes[1];

        if col < b'a' || col > b'h' {
            return Err("Invalid column in position notation");
        }
        if row < b'1' || row > b'8' {
            return Err("Invalid row in position notation");
        }

        Ok(Pos {
            col: (col - b'a') as i8,
            row: (row - b'1') as i8,
        })
    }
    pub fn col(&self) -> i8 {
        self.col
    }
    pub fn row(&self) -> i8 {
        self.row
    }
    pub fn tuple(&self) -> (i8, i8) {
        (self.col, self.row)
    }
    pub fn is_out_of_bounds(&self) -> bool {
        self.col < 0 || self.col > 7 || self.row < 0 || self.row > 7
    }
    pub fn invalid() -> Pos {
        Pos { col: -1, row: -1 }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Pos) -> Pos {
        Pos {
            col: self.col + other.col,
            row: self.row + other.row,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.col += rhs.col;
        self.row += rhs.row
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let col = char::from_u32('a' as u32 + self.col as u32).unwrap();
        let row = 1 + self.row;
        write!(f, "{}{}", col, row)
    }
}