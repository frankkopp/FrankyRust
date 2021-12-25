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

use std::fmt;
use std::str::FromStr;
use crate::Error;

#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight
}

pub const RANKS_LEN: usize = 8;

pub const RANKS: [Rank; RANKS_LEN] = [
    Rank::One,
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight
];

impl Rank {

    /// Returns the index of the rank starting at 0=one to 7=eight.
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Returns the Rank for the given index modulo 7.
    /// This means if the index is greater than 63 it is wrapped around.
    #[inline]
    pub fn from_index(i: usize) -> Rank { RANKS[(i >> 3) & 7] }

}

impl FromStr for Rank {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(Error::InvalidRank);
        }
        match s.chars().next().unwrap() {
            '1' => Ok(Rank::One  ),
            '2' => Ok(Rank::Two  ),
            '3' => Ok(Rank::Three),
            '4' => Ok(Rank::Four ),
            '5' => Ok(Rank::Five ),
            '6' => Ok(Rank::Six  ),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            _ => Err(Error::InvalidRank),
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
               Rank::One   => "1",
               Rank::Two   => "2",
               Rank::Three => "3",
               Rank::Four  => "4",
               Rank::Five  => "5",
               Rank::Six   => "6",
               Rank::Seven => "7",
               Rank::Eight => "8"
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Square;
    use super::*;

    #[test]
    fn test_index() {
        assert_eq!(0, Rank::One.to_index());
        assert_eq!(7, Rank::Eight.to_index());
    }

    #[test]
    fn test_from_index() {
        assert_eq!(Rank::One, Rank::from_index(0));
        assert_eq!(Rank::One, Rank::from_index(7));
        assert_eq!(Rank::Three, Rank::from_index(16));
        assert_eq!(Rank::Three, Rank::from_index(23));
        assert_eq!(Rank::Eight, Rank::from_index(56));
        assert_eq!(Rank::Eight, Rank::from_index(63));
        assert_eq!(Rank::One, Rank::from_index(64));
    }

    #[test]
    fn test_from_str() {
        let result = Rank::from_str("1");
        match result {
            Ok(n) => assert_eq!(Rank::One, n),
            Err(_) => panic!("Test failed")
        }

        let result = Rank::from_str("8");
        match result {
            Ok(n) => assert_eq!(Rank::Eight, n),
            Err(_) => panic!("Test failed")
        }

        let result = Rank::from_str("5");
        match result {
            Ok(n) => assert_eq!(Rank::Five, n),
            Err(_) => panic!("Test failed")
        }

        let result = Rank::from_str("9");
        match result {
            Ok(_) => panic!("Test failed"),
            Err(e) => assert_eq!(Error::InvalidRank, e),
        }
    }

    #[test]
    fn test_print() {
        assert_eq!("2", format!("{}", Rank::Two));
        assert_eq!("7", format!("{}", Rank::Seven));
    }


}
