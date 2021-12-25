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

#![allow(unused_imports)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use types::{Color, File, Rank, Square};

fn flip(color: &Color) -> Color {
    match *color {
        Color::White => Color::Black,
        Color::Black => Color::White
    }
}

fn flip2(color: &Color) -> Color {
    if *color == Color::White {
        Color::Black
    } else {
        Color::White
    }
}


fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("bench match", |b| b.iter(|| black_box(flip(&Color::Black))));
    // c.bench_function("bench if", |b| b.iter(|| black_box(flip2(&Color::Black))));

    c.bench_function("bench from1", |b| b.iter(|| black_box(File::from_index(4))));
    c.bench_function("bench from2", |b| b.iter(|| black_box(File::from_index(4))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
