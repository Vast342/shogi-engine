use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(pub u128);

use super::square::{Square, BOARD_LEN};

// a mask for a single file on the board
pub const FILEMASK: u128 =
    0b1_000000001_000000001_000000001_000000001_000000001_000000001_000000001_000000001;
// a mask for a single rank on the board
pub const RANKMASK: u128 = 0b111111111;

impl Bitboard {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self((1 << 81) - 1);

    #[must_use]
    pub const fn from_square(sq: Square) -> Self {
        Self(1 << sq.0)
    }

    #[must_use]
    pub const fn from_rank(rank: u8) -> Self {
        Self(RANKMASK << (BOARD_LEN * rank))
    }

    #[must_use]
    pub const fn from_file(file: u8) -> Self {
        Self(FILEMASK << file)
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn lsb(&self) -> u8 {
        debug_assert!(self.0 != 0, "tried to lsb an empty bitboard");
        self.0.trailing_zeros() as u8
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn msb(&self) -> u8 {
        debug_assert!(self.0 != 0, "tried to msb an empty bitboard");
        self.0.leading_zeros() as u8
    }

    pub fn pop_lsb(&mut self) -> u8 {
        let lsb: u8 = self.lsb();
        self.0 &= self.0 - 1;
        lsb
    }

    #[must_use]
    pub const fn popcount(&self) -> u32 {
        self.0.count_ones()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn is_not_empty(&self) -> bool {
        self.0 != 0
    }

    #[must_use]
    pub const fn has_bits(&self) -> bool {
        self.0 != 0
    }

    #[must_use]
    pub const fn raw(&self) -> u128 {
        self.0
    }

    #[must_use]
    pub const fn contains_multiple(&self) -> bool {
        (self.0 & self.0.wrapping_sub(1)) != 0
    }

    #[must_use]
    pub const fn contains_one(&self) -> bool {
        !self.is_empty() && !self.contains_multiple()
    }

    pub fn fill_upwards(&self) -> Bitboard {
        let mut board = Bitboard(self.0);
        board |= board << 9;
        board |= board << 18;
        board |= board << 36;
        board |= board << 72;
        board & Self::FULL
    }

    pub fn fill_downwards(&self) -> Bitboard {
        let mut board = Bitboard(self.0);
        board |= board >> 9;
        board |= board >> 18;
        board |= board >> 36;
        board |= board >> 72;
        board & Self::FULL
    }

    pub fn file_fill(&self) -> Bitboard {
        self.fill_upwards() | self.fill_downwards()
    }

    pub const fn const_and(&self, rhs: Self) -> Bitboard {
        Bitboard(self.0 & rhs.0)
    }

    pub const fn const_neg(&self) -> Bitboard {
        Bitboard(self.0.wrapping_neg())
    }

    pub const fn const_or(&self, rhs: Self) -> Bitboard {
        Bitboard(self.0 | rhs.0)
    }

    pub const fn const_sub(&self, rhs: Self) -> Bitboard {
        Bitboard(self.0 - rhs.0)
    }

    pub const fn const_shl(&self, shift: u8) -> Bitboard {
        Bitboard(self.0 << shift)
    }

    pub const fn const_xor(&self, rhs: Self) -> Bitboard {
        Bitboard(self.0 ^ rhs.0)
    }

    pub const fn hi_bits(&self) -> u64 {
        (self.0 >> 64) as u64
    }

    pub const fn lo_bits(&self) -> u64 {
        self.0 as u64
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Shr<u8> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shl<u8> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.0;
        let mut res = "".to_owned();
        for rank in (0..9).rev() {
            for file in 0..9 {
                let idx = rank * 9 + file;
                if value & (1 << idx) != 0 {
                    res += "1";
                } else {
                    res += "0";
                }
            }
            res += "\n";
        }
        write!(f, "{}", res)
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = Biterator;

    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        Biterator { board: self }
    }
}

pub struct Biterator {
    board: Bitboard,
}

impl Iterator for Biterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.board.is_empty() {
            None
        } else {
            Some(Square(self.board.pop_lsb()))
        }
    }
}
