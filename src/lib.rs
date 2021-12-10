#[macro_use]
extern crate lazy_static;

use aoc_runner_derive::aoc_lib;

mod solutions;

aoc_lib! { year = 2021, extra_alternatives = ["fnv"] }