use std::fs::File;
use std::io::prelude::*;

pub fn day01() {
    //println!("{}", std::env::current_dir().unwrap().display());
    let file = File::open("src/aoc2022/day01/input.txt");
    let mut contents = String::new();
    file.expect("file not found?").read_to_string(&mut contents).unwrap();

    let mut _tmp: Vec::<_> = contents
        .split("\n\n")
        .map(|x| x.lines().map(|l| l.parse::<i32>().unwrap())
        .sum::<i32>()
        ).collect();

    _tmp.sort();
    let first = _tmp.last().unwrap();
    let second = _tmp.iter().rev().take(3).sum::<i32>();
    
    println!("max cals: {}", first);
    println!("top 3 sum: {}", second);
}