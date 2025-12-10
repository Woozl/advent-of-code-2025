use regex::RegexBuilder;
use std::{fs, path::Path};

const NUM_TICKS: isize = 100;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-01/input.txt"))?;

    let re = RegexBuilder::new(r"^(?<dir>[RL])(?<val>\d*)$")
        .multi_line(true)
        .build()?;
    let movements: Vec<isize> = re
        .captures_iter(&input)
        .map(|caps| {
            let dir = caps.name("dir").unwrap().as_str();
            let val = caps.name("val").unwrap().as_str();

            (if dir == "R" { 1 } else { -1 }) * val.parse::<isize>().unwrap()
        })
        .collect();

    let mut location: isize = 50;
    let mut total_crossings = 0;
    for movement in movements {
        let start = location;
        location += movement;
        let end = location;

        let seg_low = std::cmp::min(start, end);
        let seg_high = std::cmp::max(start, end);

        let mult_low = next_highest_multiple_of_n(seg_low, NUM_TICKS);
        let mult_high = next_previous_multiple_of_n(seg_high, NUM_TICKS);

        let mut num_crossings = if mult_low > mult_high {
            0
        } else {
            (mult_high - mult_low) / NUM_TICKS + 1
        };
        if start % NUM_TICKS == 0 {
            num_crossings -= 1
        }

        println!(
            "s: {}\te: {}\tl: {}\th:{}\tml:{}\tmh:{}\tc:{}",
            start, end, seg_low, seg_high, mult_low, mult_high, num_crossings
        );
        total_crossings += num_crossings;
    }

    println!("Total crossings: {total_crossings}");

    Ok(())
}

fn next_highest_multiple_of_n(x: isize, n: isize) -> isize {
    if x < 0 {
        -(x.abs() / n) * n
    } else if x == 0 {
        0
    } else {
        if x % n == 0 {
            x
        } else {
            (x / n + 1) * n
        }
    }
}

fn next_previous_multiple_of_n(x: isize, n: isize) -> isize {
    if x < 0 {
        if x % n == 0 {
            x
        } else {
            -(x.abs() / n + 1) * n
        }
    } else if x == 0 {
        0
    } else {
        (x / n) * n
    }
}
