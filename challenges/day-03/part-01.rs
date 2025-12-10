use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-03/input.txt"))?;

    let banks = input.lines().map(|l| {
        l.chars()
            .map(|digit| digit.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    });

    let mut sum: u32 = 0;
    for bank in banks {
        let mut left: Vec<u32> = vec![0; bank.len()];
        let mut right: Vec<u32> = vec![0; bank.len()];

        // maximums going left to right
        let mut max: u32 = 0;
        for (i, num) in bank.iter().enumerate() {
            if *num > max {
                max = *num;
            }
            left[i] = max;
        }

        // maximums going right to left
        max = 0;
        for (i, num) in bank.iter().rev().enumerate() {
            if *num > max {
                max = *num;
            }
            right[bank.len() - 1 - i] = max;
        }

        // find largest combo
        max = 0;
        for i in 0..=left.len() - 2 {
            let joltage = format!("{}{}", left[i], right[i + 1]).parse::<u32>()?;
            if joltage > max {
                max = joltage;
            }
        }

        sum += max;
    }

    println!("Total Joltage: {}", sum);

    Ok(())
}
