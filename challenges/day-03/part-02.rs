use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-03/input.txt"))?;

    let banks = input.lines().map(|l| {
        l.chars()
            .map(|digit| digit.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    });

    Ok(())
}
