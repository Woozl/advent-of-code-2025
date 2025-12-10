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

    let mut position: isize = 50;
    let mut num_zeros: usize = 0;
    for movement in movements {
        position = position + movement;
        if position < 0 {
            position = NUM_TICKS - (position.abs() % NUM_TICKS);
            if position == NUM_TICKS {
                position = 0;
            }
        } else if position > NUM_TICKS - 1 {
            position = position % NUM_TICKS;
        }

        if position == 0 {
            num_zeros += 1
        }
    }
    println!("{num_zeros}");

    Ok(())
}
