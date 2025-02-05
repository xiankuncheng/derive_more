#![allow(dead_code)]

use derive_more::Mul;

#[derive(Mul)]
struct MyInt(i32);

#[derive(Mul)]
struct MyInts(i32, i32);

#[derive(Mul)]
struct Point1D {
    x: i32,
}

#[derive(Mul)]
struct Point2D {
    x: i32,
    y: i32,
}
