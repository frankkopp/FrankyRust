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
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

pub const FILES_LEN: usize = 8;

pub const FILES: [File; FILES_LEN] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H
];

impl File {

    /// Returns the index of the file starting at 0=A to 7=H.
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Returns the file enum for the give index modulo 7.
    /// This means that any square index can be used to get the correct file.
    #[inline]
    pub fn from_index(i: usize) -> File { FILES[i&7] }

}

impl FromStr for File {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(Error::InvalidFile);
        }
        match s.to_lowercase().chars().next().unwrap() {
            'a' => Ok(File::A),
            'b' => Ok(File::B),
            'c' => Ok(File::C),
            'd' => Ok(File::D),
            'e' => Ok(File::E),
            'f' => Ok(File::F),
            'g' => Ok(File::G),
            'h' => Ok(File::H),
            _ => Err(Error::InvalidFile),
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                 File::A => "a",
                 File::B => "b",
                 File::C => "c",
                 File::D => "d",
                 File::E => "e",
                 File::F => "f",
                 File::G => "g",
                 File::H => "h"
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        assert_eq!(0, File::A.to_index());
        assert_eq!(7, File::H.to_index());
    }

    #[test]
    fn test_from_index() {
        assert_eq!(File::A, File::from_index(0));
        assert_eq!(File::C, File::from_index(2));
        assert_eq!(File::H, File::from_index(7));
        assert_eq!(File::A, File::from_index(8));
        assert_eq!(File::H, File::from_index(63));
        assert_eq!(File::A, File::from_index(64));
    }

    #[test]
    fn test_from_str() {
        let result = File::from_str("a");
        match result {
            Ok(n) => assert_eq!(File::A, n),
            Err(_) => panic!("Test failed")
        }

        let result = File::from_str("h");
        match result {
            Ok(n) => assert_eq!(File::H, n),
            Err(_) => panic!("Test failed")
        }

        let result = File::from_str("H");
        match result {
            Ok(n) => assert_eq!(File::H, n),
            Err(_) => panic!("Test failed")
        }

        let result = File::from_str("i");
        match result {
            Ok(_) => panic!("Test failed"),
            Err(e) => assert_eq!(Error::InvalidFile, e),
        }
    }

    #[test]
    fn test_print() {
        assert_eq!("c", format!("{}", File::C));
        assert_eq!("g", format!("{}", File::G));
    }

}
