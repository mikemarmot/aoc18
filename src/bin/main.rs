use std::time::Instant;

use aoc18::day02::{p1,p2};

fn main() {
    let now = Instant::now();
    p1::doit();
    println!("   in {:?}\n", now.elapsed());

    let now = Instant::now();
    p2::doit();
    println!("   in {:?}\n", now.elapsed());
}
