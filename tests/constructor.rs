#![allow(dead_code)]
#[macro_use]
extern crate derive_more;

#[derive(Constructor)]
struct MyInts(i32, i32);

#[derive(Constructor)]
struct Point2D {
    x: i32,
    y: i32,
}
