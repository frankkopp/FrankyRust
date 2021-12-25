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

#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Color {
    White,
    Black,
}

pub const COLORS_LEN: usize = 2;

pub const COLORS: [Color; COLORS_LEN] = [Color::White,Color::Black];

impl Color {

    #[inline]
    pub fn index(&self) -> usize { *self as usize }

    #[inline]
    pub fn flip(&self) -> Color {
        match *self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }

    #[inline]
    pub fn direction(&self) -> i32 {
        match *self {
            Color::White => 1,
            Color::Black => -1
        }
    }

}

impl FromStr for Color {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(Error::InvalidColor);
        }
        match s {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => { return Err(Error::InvalidColor); }
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Color::White => "w",
                Color::Black => "b"
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        assert_eq!(0, Color::White.index());
        assert_eq!(1, Color::Black.index());
    }

    #[test]
    fn test_flip() {
        assert_eq!(Color::White, Color::Black.flip());
        assert_eq!(Color::Black, Color::White.flip());
    }

    #[test]
    fn test_from_str() {
        let result = Color::from_str("w");
        match result {
            Ok(n) => assert_eq!(Color::White, n),
            Err(_) => panic!("Test failed")
        }

        let result = Color::from_str("b");
        match result {
            Ok(n) => assert_eq!(Color::Black, n),
            Err(_) => panic!("Test failed")
        }

        let result = Color::from_str("a");
        match result {
            Ok(_) => panic!("Test failed"),
            Err(e) => assert_eq!(Error::InvalidColor, e),
        }
    }

    #[test]
    fn test_print() {
        assert_eq!("w", format!("{}", Color::White));
        assert_eq!("b", format!("{}", Color::Black));
    }

}



