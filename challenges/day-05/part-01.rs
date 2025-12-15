use std::{fs, path::Path};

use regex::RegexBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-05/input.txt"))?;

    let [ranges_str, ids_str] = input.split("\n\n").collect::<Vec<_>>().try_into().unwrap();

    let re = RegexBuilder::new(r"^(?<min>\d+)-(?<max>\d+)$")
        .multi_line(true)
        .build()?;
    let ranges: Vec<(usize, usize)> = re
        .captures_iter(ranges_str)
        .map(|caps| {
            let min = caps.name("min").unwrap().as_str().parse::<usize>().unwrap();
            let max = caps.name("max").unwrap().as_str().parse::<usize>().unwrap();
            (min, max)
        })
        .collect();

    let ids = ids_str.lines().map(|n| n.parse::<usize>().unwrap());

    let mut num_fresh: usize = 0;
    for id in ids {
        for range in ranges.iter() {
            if range.0 <= id && id <= range.1 {
                num_fresh += 1;
                break;
            }
        }
    }

    println!("{num_fresh}");
    Ok(())
}
