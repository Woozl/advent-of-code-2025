use std::{fs, path::Path};

const NUM_BATTERIES: usize = 12;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-03/input.txt"))?;

    let banks: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|digit| digit.to_digit(10).unwrap()).collect())
        .collect();

    let mut total_joltage: usize = 0;

    for bank in banks {
        let mut batteries: Vec<u32> = vec![];
        let mut curr_bat_subset: Vec<u32> = bank;

        for batteries_left in (0..NUM_BATTERIES).rev() {
            let last_idx = curr_bat_subset.len() - batteries_left;

            let search_slice = &curr_bat_subset[..last_idx];

            let max = *search_slice.iter().max().unwrap();
            let max_idx = search_slice.iter().position(|val| *val == max).unwrap();

            batteries.push(max);

            curr_bat_subset = curr_bat_subset[(max_idx + 1)..curr_bat_subset.len()].to_vec();
        }

        total_joltage += batteries
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<usize>()?;
    }

    println!("{total_joltage}");

    Ok(())
}
