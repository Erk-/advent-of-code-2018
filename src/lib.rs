#![feature(split_ascii_whitespace)]
#![allow(non_snake_case)]
#[macro_use] extern crate aoc_runner_derive;
#[macro_use] extern crate lazy_static;

pub mod utils;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

aoc_lib! {
    year = 2018
}
