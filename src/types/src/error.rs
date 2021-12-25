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

use failure::Fail;

/// Sometimes, bad stuff happens.
#[derive(Clone, Debug, Fail, PartialEq)]
pub enum Error {

    /// An attempt was made to create a color from an invalid string
    #[fail(display = "The string specified does not contain a valid notation notation for a color")]
    InvalidColor,

    /// An attempt was made to convert a string not equal to "a"-"h" to a file
    #[fail(display = "The string specified does not contain a valid file")]
    InvalidFile,

    /// An attempt was made to convert a string not equal to "1"-"8" to a rank
    #[fail(display = "The string specified does not contain a valid rank")]
    InvalidRank,

    #[fail(display = "The string specified does not contain a valid algebraic notation square")]
    InvalidSquare,

}
