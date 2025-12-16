use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-06/input.txt"))?;

    let lines = input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let (operands, number_lines) = lines.split_last().unwrap();

    let numbers: Vec<Vec<usize>> = (*number_lines)
        .into_iter()
        .map(|l| (*l).iter().map(|n| n.parse::<usize>().unwrap()).collect())
        .collect();

    let mut sum: usize = 0;
    for (i, op) in operands.iter().enumerate() {
        sum += if *op == "*" {
            numbers.iter().fold(1, |acc, n| acc * n[i])
        } else {
            numbers.iter().fold(0, |acc, n| acc + n[i])
        };
    }

    println!("{sum}");

    Ok(())
}
