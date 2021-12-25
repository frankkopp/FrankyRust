/*
 * FrankyRust
 * Copyright (c) 2022 Frank Kopp
 *
 * MIT License
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use crate::error::Error;
use std::fmt;
use std::str::FromStr;
use crate::File;
use crate::Rank;

#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8
}

pub const SQUARES_LEN: usize = 64;

pub const SQUARES: [Square; SQUARES_LEN] = [
    Square::A1,
    Square::B1,
    Square::C1,
    Square::D1,
    Square::E1,
    Square::F1,
    Square::G1,
    Square::H1,
    Square::A2,
    Square::B2,
    Square::C2,
    Square::D2,
    Square::E2,
    Square::F2,
    Square::G2,
    Square::H2,
    Square::A3,
    Square::B3,
    Square::C3,
    Square::D3,
    Square::E3,
    Square::F3,
    Square::G3,
    Square::H3,
    Square::A4,
    Square::B4,
    Square::C4,
    Square::D4,
    Square::E4,
    Square::F4,
    Square::G4,
    Square::H4,
    Square::A5,
    Square::B5,
    Square::C5,
    Square::D5,
    Square::E5,
    Square::F5,
    Square::G5,
    Square::H5,
    Square::A6,
    Square::B6,
    Square::C6,
    Square::D6,
    Square::E6,
    Square::F6,
    Square::G6,
    Square::H6,
    Square::A7,
    Square::B7,
    Square::C7,
    Square::D7,
    Square::E7,
    Square::F7,
    Square::G7,
    Square::H7,
    Square::A8,
    Square::B8,
    Square::C8,
    Square::D8,
    Square::E8,
    Square::F8,
    Square::G8,
    Square::H8
];

impl Square {

    /// Returns the index of the rank starting at 0=one to 7=eight.
    #[inline]
    pub fn to_index(&self) -> usize { *self as usize }

    /// Returns the Rank for the given index modulo 63.
    /// This means if the index is greater than 63 it is wrapped around.
    #[inline]
    pub fn from_index(i: usize) -> Square { SQUARES[i&63] }

    #[inline]
    pub fn from_file_rank(f: File, r: Rank) -> Square {
        SQUARES[(r.to_index() << 3) + f.to_index()]
    }

    /// Returns the corresponding File for the Square
    #[inline]
    pub fn file_of(&self) -> File { File::from_index(self.to_index()) }

    /// Returns the corresponding Rank for the Square
    #[inline]
    pub fn rank_of(&self) -> Rank { Rank::from_index(self.to_index()) }

}

impl FromStr for Square {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(Error::InvalidSquare);
        }

        let file = File::from_str(&s[0..1]);
        match file {
            Err(Error::InvalidFile) => return Err(Error::InvalidSquare),
            _ => ()
        }
        let rank = Rank::from_str(&s[1..2]);
        match rank {
            Err(Error::InvalidRank) => return Err(Error::InvalidSquare),
            _ => ()
        }

        Ok(Square::from_file_rank(file.unwrap(),rank.unwrap()))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            format!("{}", self.file_of()),
            format!("{}", self.rank_of())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        assert_eq!(0, Square::A1.to_index());
        assert_eq!(63, Square::H8.to_index());
    }

    #[test]
    fn test_from_index() {
        assert_eq!(Square::A1, Square::from_index(0));
        assert_eq!(Square::H1, Square::from_index(7));
        assert_eq!(Square::A3, Square::from_index(16));
        assert_eq!(Square::H3, Square::from_index(23));
        assert_eq!(Square::A8, Square::from_index(56));
        assert_eq!(Square::H8, Square::from_index(63));
        assert_eq!(Square::A1, Square::from_index(64));
    }

    #[test]
    fn test_from_file_rank() {
        assert_eq!(Square::A1, Square::from_file_rank(File::A,Rank::One));
        assert_eq!(Square::H1, Square::from_file_rank(File::H,Rank::One));
        assert_eq!(Square::E3, Square::from_file_rank(File::E,Rank::Three));
        assert_eq!(Square::F4, Square::from_file_rank(File::F, Rank::Four));
        assert_eq!(Square::A8, Square::from_file_rank(File::A,Rank::Eight));
        assert_eq!(Square::H8, Square::from_file_rank(File::H,Rank::Eight));
    }

    #[test]
    fn test_file_of() {
        assert_eq!(File::A, Square::A1.file_of());
        assert_eq!(File::H, Square::H1.file_of());
        assert_eq!(File::E, Square::E3.file_of());
        assert_eq!(File::F, Square::F7.file_of());
        assert_eq!(File::A, Square::A8.file_of());
        assert_eq!(File::H, Square::H8.file_of());
    }

    #[test]
    fn test_rank_of() {
        assert_eq!(Rank::One,   Square::A1.rank_of());
        assert_eq!(Rank::One,   Square::H1.rank_of());
        assert_eq!(Rank::Three, Square::A3.rank_of());
        assert_eq!(Rank::Three, Square::H3.rank_of());
        assert_eq!(Rank::Eight, Square::A8.rank_of());
        assert_eq!(Rank::Eight, Square::H8.rank_of());
        assert_eq!(Rank::One,   Square::A1.rank_of());
    }

    #[test]
    fn test_from_str() {
        let result = Square::from_str("a1");
        match result {
            Ok(n) => assert_eq!(Square::A1, n),
            Err(_) => panic!("Test failed")
        }

        let result = Square::from_str("A1");
        match result {
            Ok(n) => assert_eq!(Square::A1, n),
            Err(_) => panic!("Test failed")
        }

        let result = Square::from_str("f5");
        match result {
            Ok(n) => assert_eq!(Square::F5, n),
            Err(_) => panic!("Test failed")
        }

        let result = Square::from_str("h9");
        match result {
            Ok(_) => panic!("Test failed"),
            Err(e) => assert_eq!(Error::InvalidSquare, e),
        }
    }

    #[test]
    fn test_print() {
        assert_eq!("a1", format!("{}", Square::A1));
        assert_eq!("f4", format!("{}", Square::F4));
        assert_eq!("h8", format!("{}", Square::H8));
    }

}
